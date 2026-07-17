//! PhysicsLang day-1 scaffold.
//!
//! Tokens (or memory nodes) are particles. Forces between them are the working
//! model of "grammar." Ported and simplified from the curated SplatRag physics_lang
//! rebuild materials.

use nalgebra::DVector;

/// Tunable integration / force scales (subset of historical PhysicsLangKnobs).
#[derive(Debug, Clone)]
pub struct PhysicsLangKnobs {
    pub dt: f32,
    pub viscosity_scale: f32,
    pub force_cap: f32,
    pub goal_strength: f32,
    pub charge_boost: f32,
    pub spring_k_token: f32,
    pub spring_k_sentence: f32,
    pub spring_rest_adj: f32,
    pub spring_rest_dep: f32,
    pub kuramoto_coupling: f32,
    pub splat_sigma: f32,
    pub splat_alpha: f32,
}

impl Default for PhysicsLangKnobs {
    fn default() -> Self {
        Self {
            dt: 0.016,
            viscosity_scale: 0.35,
            force_cap: 7.5,
            goal_strength: 5.5,
            charge_boost: 1.0,
            spring_k_token: 0.52,
            spring_k_sentence: 0.052,
            spring_rest_adj: 1.0,
            spring_rest_dep: 3.0,
            kuramoto_coupling: 0.8,
            splat_sigma: 0.5,
            splat_alpha: 2.0,
        }
    }
}

/// Token / concept particle.
#[derive(Debug, Clone)]
pub struct PhysicsParticle {
    pub id: u64,
    pub pos: DVector<f32>,
    pub velocity: DVector<f32>,
    pub mass: f32,
    pub charge: f32,
    pub viscosity: f32,
    pub phase: f32,
    pub freq: f32,
    pub token_idx: usize,
    pub sentence_id: usize,
    pub doc_id: u64,
}

impl PhysicsParticle {
    pub fn new(id: u64, pos: DVector<f32>, mass: f32, charge: f32) -> Self {
        let dim = pos.len();
        Self {
            id,
            pos,
            velocity: DVector::zeros(dim),
            mass: mass.max(0.1),
            charge,
            viscosity: 0.35,
            phase: 0.0,
            freq: 1.0,
            token_idx: 0,
            sentence_id: 0,
            doc_id: 0,
        }
    }
}

/// Negative-mass scar: repels nearby trajectories after a bad path.
#[derive(Debug, Clone)]
pub struct SplatScar {
    pub id: u64,
    pub pos: DVector<f32>,
    pub mass: f32,
    pub viscosity: f32,
}

/// Crude POS → charge map for demos (noun −, verb +, adj +).
pub fn pos_to_charge(pos_tag: &str) -> f32 {
    match pos_tag.to_ascii_uppercase().as_str() {
        "NOUN" | "PROPN" | "NN" | "NNS" | "NNP" => -1.0,
        "VERB" | "VB" | "VBD" | "VBG" | "VBN" | "VBP" | "VBZ" => 1.0,
        "ADJ" | "JJ" | "JJR" | "JJS" => 0.5,
        _ => 0.0,
    }
}

/// Propagate a failure scar into a local neighborhood (RBF).
pub fn create_and_propagate_scar(
    bad_pos: &DVector<f32>,
    particles: &mut [PhysicsParticle],
    knobs: &PhysicsLangKnobs,
) {
    let two_sig_sq = 2.0 * knobs.splat_sigma * knobs.splat_sigma;
    for p in particles.iter_mut() {
        let dist_sq = (&p.pos - bad_pos).norm_squared();
        let weight = (-dist_sq / two_sig_sq).exp();
        if weight > 0.05 {
            p.viscosity += 2.0 * weight;
            p.mass *= 1.0 - (0.5 * weight);
            let dir = (&p.pos - bad_pos)
                .try_normalize(1e-8)
                .unwrap_or_else(|| DVector::zeros(bad_pos.len()));
            p.velocity += dir * (5.0 * weight);
        }
    }
}

fn kuramoto_step(phases: &mut [f32], freqs: &[f32], coupling: f32, dt: f32) {
    let n = phases.len();
    if n == 0 {
        return;
    }
    let mut dphase = vec![0.0f32; n];
    for i in 0..n {
        let mut s = 0.0f32;
        for j in 0..n {
            if i == j {
                continue;
            }
            s += (phases[j] - phases[i]).sin();
        }
        dphase[i] = freqs[i] + (coupling / n as f32) * s;
    }
    for i in 0..n {
        phases[i] += dphase[i] * dt;
    }
}

/// One integration step: Coulomb charge, sequence springs, optional goal, viscosity, Kuramoto.
pub fn physics_step(
    particles: &mut [PhysicsParticle],
    knobs: &PhysicsLangKnobs,
    goal: Option<&DVector<f32>>,
) {
    let n = particles.len();
    if n == 0 {
        return;
    }
    let dim = particles[0].pos.len();
    let mut acc: Vec<DVector<f32>> = (0..n).map(|_| DVector::zeros(dim)).collect();

    // 1. Coulomb / syntactic charge
    let ck = 0.8 * knobs.charge_boost;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let rij = &particles[j].pos - &particles[i].pos;
            let dsq = rij.norm_squared() + 1e-8;
            let d = dsq.sqrt();
            let qi = particles[i].charge * knobs.charge_boost;
            let qj = particles[j].charge * knobs.charge_boost;
            let f = ck * qi * qj / dsq;
            acc[i] += (f / d) * &rij;
        }
    }

    // 2. Springs (adjacency + short-range sentence)
    for i in 0..n {
        for j in (i + 1)..n {
            if particles[i].doc_id != particles[j].doc_id {
                continue;
            }
            let idx_diff = particles[j].token_idx as i32 - particles[i].token_idx as i32;
            let (k, rest) = if idx_diff.abs() == 1 {
                (knobs.spring_k_token, knobs.spring_rest_adj)
            } else if idx_diff.abs() < 8 && particles[i].sentence_id == particles[j].sentence_id {
                (knobs.spring_k_sentence * 0.1, knobs.spring_rest_dep)
            } else {
                (0.0, 0.0)
            };
            if k <= 0.0 {
                continue;
            }
            let delta = &particles[j].pos - &particles[i].pos;
            let d = delta.norm();
            if d < 1e-6 {
                continue;
            }
            let f = k * (d - rest) * (&delta / d);
            acc[i] += &f / particles[i].mass.max(0.1);
            acc[j] -= &f / particles[j].mass.max(0.1);
        }
    }

    // 3. Goal attractor
    if let Some(g) = goal {
        for i in 0..n {
            let dir = (g - &particles[i].pos)
                .try_normalize(1e-6)
                .unwrap_or_else(|| DVector::zeros(dim));
            acc[i] += dir * knobs.goal_strength;
        }
    }

    // 4. Integrate
    let dt = knobs.dt;
    for (p, a) in particles.iter_mut().zip(acc) {
        p.velocity += a * dt;
        p.velocity *= (1.0 - knobs.viscosity_scale * dt).max(0.0);
        let speed = p.velocity.norm();
        if speed > knobs.force_cap && speed > 0.0 {
            p.velocity *= knobs.force_cap / speed;
        }
        p.pos += p.velocity.clone() * dt;
    }

    // 5. Kuramoto phase sync
    if knobs.kuramoto_coupling > 0.0 {
        let mut phases: Vec<f32> = particles.iter().map(|p| p.phase).collect();
        let freqs: Vec<f32> = particles.iter().map(|p| p.freq).collect();
        kuramoto_step(&mut phases, &freqs, knobs.kuramoto_coupling, knobs.dt);
        for (p, ph) in particles.iter_mut().zip(phases) {
            p.phase = ph;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physics_step_moves_particles() {
        let knobs = PhysicsLangKnobs::default();
        let mut ps = vec![
            PhysicsParticle::new(0, DVector::from_vec(vec![0.0, 0.0, 0.0]), 1.0, -1.0),
            PhysicsParticle::new(1, DVector::from_vec(vec![1.0, 0.0, 0.0]), 1.0, 1.0),
        ];
        ps[0].token_idx = 0;
        ps[1].token_idx = 1;
        ps[0].doc_id = 1;
        ps[1].doc_id = 1;
        let before = ps[0].pos.clone();
        physics_step(&mut ps, &knobs, None);
        assert!((&ps[0].pos - before).norm() > 0.0 || ps[0].velocity.norm() > 0.0);
    }

    #[test]
    fn pos_to_charge_basic() {
        assert_eq!(pos_to_charge("NOUN"), -1.0);
        assert_eq!(pos_to_charge("VERB"), 1.0);
    }
}

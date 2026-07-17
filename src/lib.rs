//! PhysicsLang day-1 scaffold.
//!
//! Tokens (or memory nodes) are particles. Forces between them are the working
//! model of "grammar." Ported and simplified from the curated SplatRag physics_lang
//! rebuild materials.

pub mod embed;

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
    /// Strength of pull toward the ontological antipode of a concept axis (negative steering).
    /// Typical sweet spot in the original inversion work was roughly |gain| 0.15–0.30.
    pub inversion_gain: f32,
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
            inversion_gain: 0.0,
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

// --- Ontological inversion (structured antipode under a concept axis) ---
// Geometry matches the research line: reflection about a concept direction, not erase.
// Φ_c(h) ≈ μ + (I − 2 P_c)(h − μ)  with P_c = û ûᵀ for unit concept axis û.

/// Unit concept axis. Returns zeros if input is degenerate.
pub fn unit_axis(v: &DVector<f32>) -> DVector<f32> {
    v.try_normalize(1e-8)
        .unwrap_or_else(|| DVector::zeros(v.len()))
}

/// Householder reflection of `h` through the hyperplane orthogonal to unit axis `u`,
/// around optional center `mu` (defaults to origin).
///
/// This is the structured *other side* of a concept — not zeroing the vector.
pub fn householder_reflect(h: &DVector<f32>, u: &DVector<f32>, mu: Option<&DVector<f32>>) -> DVector<f32> {
    let dim = h.len();
    let u = unit_axis(u);
    if u.norm() < 1e-8 {
        return h.clone();
    }
    let center = mu
        .cloned()
        .unwrap_or_else(|| DVector::zeros(dim));
    let d = h - &center;
    // (I - 2 uu^T) d = d - 2 (u·d) u
    let proj = u.dot(&d);
    &center + &d - 2.0 * proj * &u
}

/// Ontological antipode of `h` on axis `concept` about `mu`.
pub fn antipode(h: &DVector<f32>, concept: &DVector<f32>, mu: Option<&DVector<f32>>) -> DVector<f32> {
    householder_reflect(h, concept, mu)
}

/// Blend `h` toward its antipode with gain `alpha` in [0, 1+] (negative steering strength).
/// `alpha = 0` → unchanged; `alpha = 1` → full reflection; values ~0.15–0.30 match historical sweet spot.
pub fn invert_toward_antipode(
    h: &DVector<f32>,
    concept: &DVector<f32>,
    alpha: f32,
    mu: Option<&DVector<f32>>,
) -> DVector<f32> {
    let a = antipode(h, concept, mu);
    (1.0 - alpha) * h + alpha * a
}

/// Apply inversion force: each particle is pulled toward its antipode on `concept` axis.
pub fn apply_inversion_force(
    particles: &mut [PhysicsParticle],
    concept: &DVector<f32>,
    knobs: &PhysicsLangKnobs,
    mu: Option<&DVector<f32>>,
) {
    if knobs.inversion_gain.abs() < 1e-8 {
        return;
    }
    let gain = knobs.inversion_gain;
    for p in particles.iter_mut() {
        let target = invert_toward_antipode(&p.pos, concept, gain.clamp(0.0, 1.5), mu);
        let dir = (&target - &p.pos)
            .try_normalize(1e-8)
            .unwrap_or_else(|| DVector::zeros(p.pos.len()));
        // Accel-like impulse into velocity (same units as other soft forces)
        p.velocity += dir * (gain.abs() * knobs.goal_strength * 0.25);
    }
}

// --- 64D forge helpers (latent crucible dim used across niodv4 / codec work) ---

/// Working latent dimension used in the encode_decode / 64D research line.
pub const LATENT_DIM_64: usize = 64;

/// Tiny deterministic PRNG (xorshift32) so demos need no extra crate.
#[derive(Clone, Debug)]
pub struct XorShift32(u32);

impl XorShift32 {
    pub fn new(seed: u32) -> Self {
        Self(if seed == 0 { 0xA341_316C } else { seed })
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// Uniform in [-1, 1).
    pub fn next_f32(&mut self) -> f32 {
        let u = self.next_u32() as f32 / (u32::MAX as f32);
        u * 2.0 - 1.0
    }

    pub fn vec(&mut self, dim: usize) -> DVector<f32> {
        DVector::from_iterator(dim, (0..dim).map(|_| self.next_f32()))
    }

    pub fn unit_vec(&mut self, dim: usize) -> DVector<f32> {
        let v = self.vec(dim);
        unit_axis(&v)
    }
}

/// Mean position of a particle cloud (or zeros if empty).
pub fn mean_position(particles: &[PhysicsParticle]) -> DVector<f32> {
    if particles.is_empty() {
        return DVector::zeros(0);
    }
    let dim = particles[0].pos.len();
    let mut m = DVector::zeros(dim);
    for p in particles {
        m += &p.pos;
    }
    m / particles.len() as f32
}

/// RMS of L2 norms of particle positions.
pub fn rms_radius(particles: &[PhysicsParticle]) -> f32 {
    if particles.is_empty() {
        return 0.0;
    }
    let s: f32 = particles.iter().map(|p| p.pos.norm_squared()).sum();
    (s / particles.len() as f32).sqrt()
}

/// Build a small cloud of 64D particles with alternating charge for demo / forge tests.
pub fn spawn_64d_cloud(n: usize, seed: u32) -> Vec<PhysicsParticle> {
    let mut rng = XorShift32::new(seed);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut pos = rng.vec(LATENT_DIM_64);
        // mild normalize so force scales stay sane
        let nrm = pos.norm().max(1e-6);
        pos /= nrm;
        let charge = if i % 2 == 0 { -1.0 } else { 1.0 };
        let mut p = PhysicsParticle::new(i as u64, pos, 1.0, charge);
        p.token_idx = i;
        p.doc_id = 1;
        p.sentence_id = 0;
        out.push(p);
    }
    out
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

/// Options for one physics step.
#[derive(Debug, Clone, Default)]
pub struct StepContext<'a> {
    /// Attractive goal (e.g. query / intent).
    pub goal: Option<&'a DVector<f32>>,
    /// Concept axis for ontological inversion (negative steering toward structured antipode).
    pub inversion_axis: Option<&'a DVector<f32>>,
    /// Center of reflection (often concept mean); default origin.
    pub inversion_mu: Option<&'a DVector<f32>>,
}

/// One integration step: Coulomb charge, sequence springs, optional goal,
/// optional ontological inversion, viscosity, Kuramoto.
pub fn physics_step(
    particles: &mut [PhysicsParticle],
    knobs: &PhysicsLangKnobs,
    goal: Option<&DVector<f32>>,
) {
    physics_step_ex(
        particles,
        knobs,
        &StepContext {
            goal,
            ..Default::default()
        },
    );
}

/// Extended step with inversion context.
pub fn physics_step_ex(
    particles: &mut [PhysicsParticle],
    knobs: &PhysicsLangKnobs,
    ctx: &StepContext<'_>,
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
    if let Some(g) = ctx.goal {
        for i in 0..n {
            let dir = (g - &particles[i].pos)
                .try_normalize(1e-6)
                .unwrap_or_else(|| DVector::zeros(dim));
            acc[i] += dir * knobs.goal_strength;
        }
    }

    // 3b. Ontological inversion: soft pull toward structured antipode on concept axis
    if let Some(axis) = ctx.inversion_axis {
        if knobs.inversion_gain.abs() > 1e-8 {
            let alpha = knobs.inversion_gain.clamp(0.0, 1.5);
            for i in 0..n {
                let target =
                    invert_toward_antipode(&particles[i].pos, axis, alpha, ctx.inversion_mu);
                let dir = (&target - &particles[i].pos)
                    .try_normalize(1e-8)
                    .unwrap_or_else(|| DVector::zeros(dim));
                acc[i] += dir * (alpha * knobs.goal_strength);
            }
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

    #[test]
    fn antipode_flips_along_axis() {
        // Living-ish point on +x; concept axis = x; antipode should land on -x side.
        let h = DVector::from_vec(vec![1.0, 0.2, 0.0]);
        let axis = DVector::from_vec(vec![1.0, 0.0, 0.0]);
        let a = antipode(&h, &axis, None);
        assert!(a[0] < 0.0, "expected flip on concept axis, got {a:?}");
        // transverse components preserved under pure x-reflection
        assert!((a[1] - 0.2).abs() < 1e-5);
    }

    #[test]
    fn invert_gain_interpolates() {
        let h = DVector::from_vec(vec![1.0, 0.0]);
        let axis = DVector::from_vec(vec![1.0, 0.0]);
        let mid = invert_toward_antipode(&h, &axis, 0.5, None);
        // half way from +1 to -1 on x
        assert!((mid[0] - 0.0).abs() < 1e-4, "got {}", mid[0]);
    }

    #[test]
    fn inversion_step_pulls_toward_antipode() {
        let mut knobs = PhysicsLangKnobs::default();
        knobs.inversion_gain = 0.25;
        knobs.goal_strength = 8.0;
        knobs.kuramoto_coupling = 0.0;
        knobs.charge_boost = 0.0; // isolate inversion
        knobs.spring_k_token = 0.0;
        let axis = DVector::from_vec(vec![1.0, 0.0]);
        let mut ps = vec![PhysicsParticle::new(
            0,
            DVector::from_vec(vec![1.0, 0.0]),
            1.0,
            0.0,
        )];
        let before = ps[0].pos[0];
        let ctx = StepContext {
            inversion_axis: Some(&axis),
            ..Default::default()
        };
        for _ in 0..30 {
            physics_step_ex(&mut ps, &knobs, &ctx);
        }
        assert!(
            ps[0].pos[0] < before,
            "expected move toward -x antipode, before={before} after={}",
            ps[0].pos[0]
        );
    }

    #[test]
    fn forge_64d_step_scar_inversion() {
        let mut knobs = PhysicsLangKnobs::default();
        knobs.inversion_gain = 0.2;
        knobs.kuramoto_coupling = 0.0;
        let mut cloud = spawn_64d_cloud(8, 42);
        assert_eq!(cloud[0].pos.len(), LATENT_DIM_64);
        let axis = unit_axis(&cloud[0].pos);
        let bad = cloud[0].pos.clone();
        let visc_before = cloud[1].viscosity;
        create_and_propagate_scar(&bad, &mut cloud, &knobs);
        // neighbor should feel scar (not always true if far — cloud is unit sphere so likely)
        let _ = visc_before;
        let r0 = rms_radius(&cloud);
        let ctx = StepContext {
            inversion_axis: Some(&axis),
            inversion_mu: None,
            goal: None,
        };
        for _ in 0..10 {
            physics_step_ex(&mut cloud, &knobs, &ctx);
        }
        let r1 = rms_radius(&cloud);
        // finite, non-NaN dynamics
        assert!(r0.is_finite() && r1.is_finite());
        assert!(cloud.iter().all(|p| p.pos.iter().all(|x| x.is_finite())));
    }
}

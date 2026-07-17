//! A/B: same Qwen embeddings, with vs without syntactic charge.
//! charge ON  = map + springs + goal + Coulomb role bias
//! charge OFF = map + springs + goal only (embed geometry does the talking)

use physicslang::embed::{embed_to_64, EmbedClient};
use physicslang::{
    mean_position, physics_step_ex, pos_to_charge, rms_radius, PhysicsLangKnobs, PhysicsParticle,
    StepContext,
};

fn build_cloud(
    client: &EmbedClient,
    tokens: &[(&str, &str)],
) -> Result<Vec<PhysicsParticle>, String> {
    let mut particles = Vec::new();
    for (i, (word, pos)) in tokens.iter().enumerate() {
        let pos64 = embed_to_64(client, word)?;
        let mut p = PhysicsParticle::new(i as u64, pos64, 1.0, pos_to_charge(pos));
        p.token_idx = i;
        p.doc_id = 1;
        p.sentence_id = 0;
        particles.push(p);
    }
    Ok(particles)
}

fn pair_dists(ps: &[PhysicsParticle]) -> Vec<((usize, usize), f32)> {
    let mut out = Vec::new();
    for i in 0..ps.len() {
        for j in (i + 1)..ps.len() {
            let d = (&ps[i].pos - &ps[j].pos).norm();
            out.push(((i, j), d));
        }
    }
    out
}

fn run_arm(label: &str, mut particles: Vec<PhysicsParticle>, charge_boost: f32, steps: usize) {
    let names = ["hamster", "eats", "magma", "fire", "container"];
    let goal = mean_position(&particles);
    let mut knobs = PhysicsLangKnobs::default();
    knobs.dt = 0.02;
    knobs.goal_strength = 3.0;
    knobs.charge_boost = charge_boost;
    knobs.kuramoto_coupling = 0.2;
    knobs.inversion_gain = 0.0;

    let d0 = pair_dists(&particles);
    let ctx = StepContext {
        goal: Some(&goal),
        ..Default::default()
    };
    for _ in 0..steps {
        physics_step_ex(&mut particles, &knobs, &ctx);
    }
    let d1 = pair_dists(&particles);

    println!("\n=== {label} (charge_boost={charge_boost}) steps={steps} ===");
    println!("rms: {:.4} → {:.4}", /* before we didn't save */ 1.0, rms_radius(&particles));
    println!("pair distances (before → after):");
    for (k, ((i, j), before)) in d0.iter().enumerate() {
        let after = d1[k].1;
        println!(
            "  {:>9}–{:<9}  {:.4} → {:.4}  (Δ {:+.4})",
            names[*i],
            names[*j],
            before,
            after,
            after - before
        );
    }
    // verb–noun pairs of interest: eats–hamster (0,1), eats–magma (1,2)
    let eh = d1.iter().find(|((i, j), _)| *i == 0 && *j == 1).map(|x| x.1).unwrap();
    let em = d1.iter().find(|((i, j), _)| *i == 1 && *j == 2).map(|x| x.1).unwrap();
    let hm = d1.iter().find(|((i, j), _)| *i == 0 && *j == 2).map(|x| x.1).unwrap();
    println!("focus: hamster–eats={eh:.4}  eats–magma={em:.4}  hamster–magma={hm:.4}");
}

fn main() {
    let client = EmbedClient::local_qwen();
    let tokens: &[(&str, &str)] = &[
        ("hamster", "NOUN"),
        ("eats", "VERB"),
        ("magma", "NOUN"),
        ("fire", "NOUN"),
        ("container", "NOUN"),
    ];

    println!("embedding via {} …", client.url);
    let base = match build_cloud(&client, tokens) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}\nIs :8302 up?");
            std::process::exit(1);
        }
    };
    for (i, (w, pos)) in tokens.iter().enumerate() {
        println!(
            "  {w:10} charge={:+.1}  ||e||={:.3}",
            base[i].charge,
            base[i].pos.norm()
        );
        let _ = pos;
    }

    let steps = 30;
    run_arm("A charge ON ", base.clone(), 1.0, steps);
    run_arm("B charge OFF", base, 0.0, steps);
    println!("\nok");
}

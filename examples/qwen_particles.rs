//! Embed real words with local Qwen3-Embedding-8B (llama-server :8302),
//! project 4096D → 64D, run PhysicsLang step with goal + optional inversion.

use nalgebra::DVector;
use physicslang::embed::{embed_to_64, EmbedClient};
use physicslang::{
    mean_position, physics_step_ex, pos_to_charge, rms_radius, PhysicsLangKnobs, PhysicsParticle,
    StepContext,
};

fn main() {
    let client = EmbedClient::local_qwen();
    // noun / verb / noun-ish toy phrase pieces
    let tokens: &[(&str, &str)] = &[
        ("hamster", "NOUN"),
        ("eats", "VERB"),
        ("magma", "NOUN"),
        ("fire", "NOUN"),
        ("container", "NOUN"),
    ];

    println!("embedding via {} …", client.url);
    let mut particles = Vec::new();
    for (i, (word, pos)) in tokens.iter().enumerate() {
        match embed_to_64(&client, word) {
            Ok(pos64) => {
                let mut p = PhysicsParticle::new(i as u64, pos64, 1.0, pos_to_charge(pos));
                p.token_idx = i;
                p.doc_id = 1;
                p.sentence_id = 0;
                println!(
                    "  {word:10} charge={:+.1} 64d[0..4]={:?}",
                    p.charge,
                    (0..4).map(|k| format!("{:.3}", p.pos[k])).collect::<Vec<_>>()
                );
                particles.push(p);
            }
            Err(e) => {
                eprintln!("embed failed for '{word}': {e}");
                eprintln!("Is llama-server --embeddings up on :8302?");
                std::process::exit(1);
            }
        }
    }

    let goal = mean_position(&particles);
    let mut knobs = PhysicsLangKnobs::default();
    knobs.dt = 0.02;
    knobs.goal_strength = 3.0;
    knobs.inversion_gain = 0.0; // set >0 and pass axis to invert
    knobs.kuramoto_coupling = 0.2;

    // Concept axis ≈ first particle (living/creature side); flip toward antipode if enabled
    let axis = particles[0].pos.clone();
    let ctx = StepContext {
        goal: Some(&goal),
        inversion_axis: None,
        inversion_mu: Some(&goal),
    };

    println!(
        "t=0  rms={:.4}  goal_norm={:.4}",
        rms_radius(&particles),
        goal.norm()
    );
    for step in 1..=30 {
        physics_step_ex(&mut particles, &knobs, &ctx);
        if step % 10 == 0 {
            println!("t={step:02} rms={:.4}", rms_radius(&particles));
        }
    }

    // Optional: one inversion pass demo
    let mut knobs_inv = knobs.clone();
    knobs_inv.inversion_gain = 0.2;
    let ctx_inv = StepContext {
        goal: Some(&goal),
        inversion_axis: Some(&axis),
        inversion_mu: Some(&goal),
    };
    let before = particles[0].pos.clone();
    for _ in 0..15 {
        physics_step_ex(&mut particles, &knobs_inv, &ctx_inv);
    }
    let delta = (&particles[0].pos - &before).norm();
    println!("after inversion pull: p0 moved Δ={delta:.4}");
    println!("ok");
    let _ = DVector::<f32>::zeros(0); // keep import happy if unused in some builds
}

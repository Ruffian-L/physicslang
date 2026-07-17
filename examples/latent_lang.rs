//! PhysicsLang as *latent* language — not English.
//!
//! Particles are points in the 64D forge (hidden/latent states), not words with POS.
//! "Grammar" here = how latents bond, scar, and invert under forces.
//! Human text is only an optional way to *seed* latents via embeddings.

use physicslang::embed::{embed_to_64, EmbedClient};
use physicslang::{
    antipode, create_and_propagate_scar, mean_position, physics_step_ex, rms_radius,
    spawn_64d_cloud, unit_axis, PhysicsLangKnobs, PhysicsParticle, StepContext, LATENT_DIM_64,
};

fn seed_from_text(client: &EmbedClient, phrases: &[&str]) -> Result<Vec<PhysicsParticle>, String> {
    let mut out = Vec::new();
    for (i, phrase) in phrases.iter().enumerate() {
        // phrase → latent seed (no POS / no "noun verb")
        let z = embed_to_64(client, phrase)?;
        let mut p = PhysicsParticle::new(i as u64, z, 1.0, 0.0); // charge 0: no human-syntax Coulomb
        p.token_idx = i;
        p.doc_id = 1;
        p.sentence_id = 0;
        out.push(p);
    }
    Ok(out)
}

fn main() {
    println!("PhysicsLang · latent language  dim={LATENT_DIM_64}");
    println!("particles = latent states; forces = latent grammar\n");

    let client = EmbedClient::local_qwen();
    // Seeds are *meaning blobs* in latent space — not a parse tree
    let phrases = [
        "living creature warmth",
        "eating consuming energy",
        "molten rock heat",
        "inanimate vessel container",
    ];

    let mut cloud = match seed_from_text(&client, &phrases) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("embed unavailable ({e}); falling back to synthetic 64D cloud");
            spawn_64d_cloud(4, 64)
        }
    };

    for (i, ph) in phrases.iter().enumerate().take(cloud.len()) {
        println!("  z[{i}] seed={ph:?}  ||z||={:.3}", cloud[i].pos.norm());
    }

    let mu = mean_position(&cloud);
    // Concept axis: first latent ("living") — inversion pushes toward structured other side
    let living_axis = unit_axis(&cloud[0].pos);
    let anti0 = antipode(&cloud[0].pos, &living_axis, Some(&mu));
    println!(
        "\nantipode check: z0·axis={:.3}  anti0·axis={:.3}  (should flip sign)",
        cloud[0].pos.dot(&living_axis),
        anti0.dot(&living_axis)
    );

    let mut knobs = PhysicsLangKnobs::default();
    knobs.dt = 0.02;
    knobs.charge_boost = 0.0; // no human POS grammar
    knobs.spring_k_token = 0.35; // sequence in *latent stream* order only
    knobs.goal_strength = 2.5;
    knobs.inversion_gain = 0.0;
    knobs.kuramoto_coupling = 0.15;
    knobs.splat_sigma = 0.6;

    // Phase A: pure latent dynamics (springs + goal + phase) — latent "syntax"
    let goal = mu.clone();
    let ctx = StepContext {
        goal: Some(&goal),
        ..Default::default()
    };
    println!("\n--- A: latent field only (no inversion) ---");
    println!("t=0  rms={:.4}", rms_radius(&cloud));
    for step in 1..=25 {
        physics_step_ex(&mut cloud, &knobs, &ctx);
        if step % 5 == 0 {
            println!("t={step:02} rms={:.4}", rms_radius(&cloud));
        }
    }

    // Scar a bad latent path (failure in latent language, not English)
    let bad = cloud[0].pos.clone();
    create_and_propagate_scar(&bad, &mut cloud, &knobs);
    println!(
        "scar at z0 → neighbor viscosity z1={:.3}",
        cloud.get(1).map(|p| p.viscosity).unwrap_or(0.0)
    );

    // Phase B: ontological inversion on living axis (negative gain in latent space)
    knobs.inversion_gain = 0.22;
    let ctx_inv = StepContext {
        goal: Some(&goal),
        inversion_axis: Some(&living_axis),
        inversion_mu: Some(&goal),
    };
    let z0_before = cloud[0].pos.dot(&living_axis);
    println!("\n--- B: inversion gain on living axis ---");
    for step in 1..=25 {
        physics_step_ex(&mut cloud, &knobs, &ctx_inv);
        if step % 5 == 0 {
            let proj: Vec<String> = cloud
                .iter()
                .map(|p| format!("{:.3}", p.pos.dot(&living_axis)))
                .collect();
            println!(
                "t={step:02} proj_on_living=[{}] rms={:.4}",
                proj.join(", "),
                rms_radius(&cloud)
            );
        }
    }
    let z0_after = cloud[0].pos.dot(&living_axis);
    println!(
        "\nz0 projection on living axis: {:.4} → {:.4}  (Δ {:+.4})",
        z0_before,
        z0_after,
        z0_after - z0_before
    );
    println!("ok · latent language step complete");
}

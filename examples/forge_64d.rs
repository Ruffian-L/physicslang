//! 64D latent crucible demo: particle cloud + failure scar + ontological inversion.
//! Matches the working dim used in encode_decode / niodv4 research (64D forge).

use physicslang::{
    create_and_propagate_scar, mean_position, physics_step_ex, rms_radius, spawn_64d_cloud,
    unit_axis, PhysicsLangKnobs, StepContext, LATENT_DIM_64,
};

fn main() {
    println!("PhysicsLang 64D forge  dim={LATENT_DIM_64}");

    let mut knobs = PhysicsLangKnobs::default();
    knobs.dt = 0.02;
    knobs.inversion_gain = 0.22; // signed field (gain), sweet-spot-ish
    knobs.goal_strength = 4.0;
    knobs.kuramoto_coupling = 0.3;
    knobs.splat_sigma = 0.8;

    let mut cloud = spawn_64d_cloud(16, 20260501); // seed nods at shadowvault forge note date
    let axis = unit_axis(&cloud[0].pos);
    let mu = mean_position(&cloud);

    println!(
        "t=0  rms_r={:.4}  mean_norm={:.4}  p0_charge={:.1}",
        rms_radius(&cloud),
        mu.norm(),
        cloud[0].charge
    );

    // Drop a scar at particle 0 ("bad path") — negative-mass style conscience field
    let bad = cloud[0].pos.clone();
    create_and_propagate_scar(&bad, &mut cloud, &knobs);
    println!(
        "scar  neighbor viscosity p1={:.3} (was ~0.35)",
        cloud[1].viscosity
    );

    let ctx = StepContext {
        goal: Some(&mu), // soft return to cloud center
        inversion_axis: Some(&axis),
        inversion_mu: Some(&mu),
    };

    for step in 1..=50 {
        physics_step_ex(&mut cloud, &knobs, &ctx);
        if step % 10 == 0 {
            let m = mean_position(&cloud);
            println!(
                "t={step:02} rms_r={:.4} mean_norm={:.4} p0_x0={:.4}",
                rms_radius(&cloud),
                m.norm(),
                cloud[0].pos[0]
            );
        }
    }

    // All finite?
    let ok = cloud.iter().all(|p| p.pos.iter().all(|x| x.is_finite()));
    println!("{}", if ok { "ok (finite 64D dynamics)" } else { "FAIL non-finite" });
}

# Splat / Scar Logic — The Conscience & Learning Engine (SplatRAG heart)

From multiple convos (Niodoo-TCS Dreaming, PhysicsLang, containerizing, etc.):

Key insight: "The Governor is a crutch... To teach it, you must restore the SplatRAG volumetric memory. ... Every failure drops a negative-mass scar. ... physics physically repels it. It learns because deviation would violate its own semantic momentum conservation."

Core function (cleaned):
```rust
pub fn create_splat_and_propagate(bad_pos: nalgebra::Vector3<f32>, memory: &mut SplatMemory) {
    let sigma = 0.5f32;
    let two_sig_sq = 2.0 * sigma * sigma;
    let mut scar = Splat::default();
    scar.position = bad_pos;
    scar.mass = -2.0;         // Negative mass (Repulsor)
    scar.viscosity = 5.0;     // Instant Hydraulic Jump / Friction
    let scar_id = memory.insert(scar);

    for splat in memory.iter_mut() {
        if splat.id == scar_id { continue; }
        let dist_sq = (splat.position - bad_pos).norm_squared();
        let weight = (-dist_sq / two_sig_sq).exp();
        if weight > 0.05 {
            splat.viscosity += 2.0 * weight;
            splat.mass *= 1.0 - (0.5 * weight);
            if let Some(vel) = &mut splat.velocity {
                let dir = (splat.position - bad_pos).normalize();
                *vel += dir * (5.0 * weight);
            }
            if weight > 0.80 {
                splat.connected_edges.retain(|&e| e != scar_id);
            }
        }
    }
}
```

Tag actions that modulate the physics (from "FlinchDetector" + tag system — map to your regulation/emergence_controller or [FOCUS]/[EXPLORE] tags in generation):
- FOCUS: cool noise, strong gravity lock, higher viscosity, slow dt
- EXPLORE: high noise, low gravity, strong repulsion, superfluid (low visc)
- SPIKE: violent recoil, turn gravity off, max repulsion (shatter loops)

In current SplatRag: look at src/bin/dream.rs, src/regulation/, src/physics/mitosis.rs, src/learning/ for places to call scar creation when TDA or homeostasis detects "bad" states.

Also: positive splats / evaporation for good paths (Anchor Splats with variable viscosity).

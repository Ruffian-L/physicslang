# Porting / Integration Map: PhysicsLang Concepts → Current SplatRag Crates

| Historical Concept (PhysicsLang/GravGram) | Current Location | Status | Action for Rebuild |
|-------------------------------------------|------------------|--------|--------------------|
| Token/Word as Particle (pos=embed, mass=IDF, charge=POS) | SemanticGaussian (mean, scaling/rot, entropy) + structs | Partial (Gaussians good for extent/viscosity) | Add explicit Particle struct or extend SemanticGaussian with mass, charge, velocity, springs. Or separate TokenParticle for generative phase. |
| Force laws (gravity, Coulomb/charge syntax, springs for grammar) | gpu_engine.rs (query on Gaussians), tissue | Rudimentary (radiance, precision) | Port the N-body / z-order force kernels or CPU sim. Add charge-gated, spring (adjacency + dep), negative-mass repulsion. Wire into generation. |
| Kuramoto + phase sync for binding + temporal grammar | generative/oscillatory_network.rs + neuron | Good skeleton (rhythm, inhibition, refractory) | Extend with per-neuron "charge", coupling matrix from syntax/adj, full Kuramoto update (sin phase diff). Use phase diff for "binding strength". |
| OU dynamics, viscosity, dt, sigma, temperature/annealing | Various (perceptual/phase_locked, learning/pinn?, config) | Partial | Centralize in a PhysicsParams or expand PhysicsKnobs + SimParams. Add update_ou, apply_viscosity. |
| Splat / Scar creation + propagation (negative mass trauma, viscosity leak, sever edges) | memory/ (core_memories, topological), storage/, physics/tissue | SplatGeometry, SplatBlobStore exist; scars mentioned in history | Implement create_splat_scar(bad_pos, memory) that inserts negative-mass Gaussian or special Splat with viscosity boost + propagation to neighbors. Use in "Dream" or error paths (see src/bin/dream.rs, update_rules). |
| Goal / query as strong attractor (black hole mass) | retrieval/ , physics (Radiance) | query_precision_boost exists | Add "goal_strength", "blackhole_mass" knob; during generative physics, inject a virtual high-mass goal particle that everything is pulled toward (or repelled for exploration). |
| GPU accel (morton, z-order forces, CUDA kernels) | src/gpu/ (lophat, cuda), physics/gpu_engine | Strong (Candle + custom CUDA for TDA) | Extend for particle forces. Or keep CPU sim for small N (per-sentence tokens ~20-100) + GPU for memory splats (N=10k+). |
| Emergent grammar via physics (SVO as geodesic) | None explicit | New | After physics relaxation step on candidate tokens, project final positions/velocities back to vocab (nearest neighbor or energy) to pick next token. Or blend force-derived "steering" into logits. |
| TDA / Betti for loop detection → inject scar/temp | src/indexing/persistent_homology, gpu/lophat, learning/tda_engine, regulation/ | Excellent (lophat, betti) | Wire: after micro-dream or generation segment, run TDA on trajectory → if b1 high (loop), trigger scar or raise temperature in generative physics. |
| Config (rich physics params) | splat_config.toml + src/config.rs (limited PhysicsKnobs) | Needs expansion | Add full PhysicsConfig from history: dt, viscosity_scale, force_cap, splat_*, goal_strength, syntactic_charge_*, spring_k_*, kuramoto_*, etc. Version it. |
| "Watcher" / conscience / Dream loop | src/bin/dream.rs, regulation/, watch.rs, shadow_daemon | Partial (homeostasis, emergence_controller) | Make the physics the teacher: failures → splats. Success → positive mass or evaporation. |

**Priority Order for Rebuild** (suggested):
1. Expand PhysicsKnobs + load from toml (add the historical rich set).
2. Core Particle + force accumulator (CPU first for clarity) in src/physics/ or new src/physics_lang/.
3. Wire basic gravity + charge + spring forces into a "relax_tokens" or "physics_step" that can be called from generative controller.
4. Enhance OscillatoryNetwork with charge/spring/Kuramoto coupling (map tokens → neurons or run parallel).
5. Implement splat_scar creation + propagation (reuse SemanticGaussian or SplatGeometry).
6. Close the loop: use physics state to influence token selection / hidden steering.
7. GPU port of forces + TDA-triggered scars.
8. Dream/Watcher integration for online "scarring" during self-recursion.

See individual curated_*.md and .rs files for the exact code to port.

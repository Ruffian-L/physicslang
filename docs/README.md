# Curated Rebuild Artifacts for PhysicsLang / GravitationGrammar in SplatRag

**Status (updated for full CUDA + full rebuild)**: Infra (Cargo gpu-accel, kernels/physics_forces.cu with history ports, KernelCache, GpuPhysicsEngine in src/gpu/physics.rs + dispatch), physics_lang GPU/CPU, wiring notes in generative/regulation/dream/physics. Pre-existing dense parse issues (embeddings/encoder/...) acknowledged; our files clean. See plan.md for details. Run with SPLATRAG_USE_GPU=1 for CUDA path.

## Files
- 00_NORTHSTAR_PhysicsLang_GravitationGrammar.md : the big picture + thesis (read first)
- 01_porting_map.md : exact mapping to your current files (physics/, generative/, regulation/, etc.) + priorities
- 02_expanded_physics_knobs.toml : the rich param set (now also in splat_config.toml [physics_lang] + src/config.rs PhysicsLangKnobs)
- 03_curated_forces_and_particles.rs : cleaned best-of Rust code + full recommended struct + integration skeleton (primary coding reference)
- 04_splat_scar_logic.md : the negative-mass learning / conscience mechanism (SplatRAG heart)
- 05_best_raw_snippets.md : 25 substantial raw (lightly filtered) blocks from the 593 for deeper variants
- 06_next_steps.md : concrete immediate actions while rebuilding

Also consult the sibling `../extracted/` (and its README) for the full raw volume if you need more history or variants.

## What was done in this curation pass
- Extracted + heavily filtered the relevant PhysicsLang / GravitationGrammar / SplatRAG code + discussion from the 600MB prod-grok-backend.json.
- Synthesized a clean "Northstar" vision.
- Produced a porting map against the *actual current* SplatRag source tree (SemanticGaussian, OscillatoryNetwork, GpuTissue, LegacyPhysicsConfig, TDA, dream/regulation bins, etc.).
- Expanded the live config (HyperParameters now carries PhysicsLangKnobs; toml has the section; LegacyPhysicsConfig left in place for memory splats).
- Landed a new `src/physics_lang/` module with PhysicsParticle, SplatScar, physics_step (charge + springs + goal + viscosity), create_and_propagate_scar, Kuramoto helper, pos_to_charge, and a smoke test. Wired into lib.rs.
- All new code is balanced, documented, and references the history.

## Quick start while rebuilding
```bash
# after you fix any pre-existing dense-file issues in the tree:
cargo test physics_lang --lib   # the smoke test in the new module
# or
cargo run --bin splat_cli -- ...  # once you wire the knobs/stepper into a path
```

Next step recommendation (see 06_next_steps.md):
1. Make the tree compile cleanly (fmt / repair embeddings.rs etc. if the long-line parser chokes).
2. Add a small test that loads HyperParameters, builds 8-16 PhysicsParticles from a toy sentence (use your embedder), assigns charges via pos_to_charge or hardcoded, runs physics_step N times with a goal, prints positions before/after.
3. Cross-link the oscillatory_network into the same dynamics (phases + springs).
4. Drop scars from a dream or homeostasis failure path.

We are rebuilding the *soul* — where syntax and meaning crystallize out of force laws instead of being prompted or sampled. Physics *is* the grammar.

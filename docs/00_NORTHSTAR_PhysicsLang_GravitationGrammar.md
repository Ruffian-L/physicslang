# PhysicsLang + GravitationGrammar + SplatRAG — Curated Northstar for Rebuild (2026)

**Origin**: Synthesized across 100s of conversations in prod-grok-backend.json (Niodoo-TCS, PhysicsLang series, hydrodynamic-swarm, SplatRAG, token physics, etc.)

**Core Thesis**:
Language generation is not sampling from a distribution.
It is **physical crystallization** of meaning in a high-dimensional semantic manifold.
Grammar, syntax, coherence, and even "thought" are **emergent properties of force laws** acting on token-particles.

**Key Primitives (from history)**:
- **Particle / Token**: position (embedding, often 512D or Matryoshka), mass (IDF / surprisal / rarity — rare words are heavy), velocity, charge (syntactic from POS: verb +1, noun -1, adj +0.5, etc. for charge-gated forces), viscosity/friction.
- **Splat / Scar (SplatRAG memory)**: When the system errs or takes low-energy bad path, drop a **negative-mass scar** at that semantic location. It creates a repulsive "conscience" field. Future trajectories are physically repelled. Learning = scarring the manifold. Viscosity transfers trauma to neighbors.
- **Forces that *are* the grammar**:
  - Gravity (global attraction to dense meaning clusters)
  - Coulomb / charge forces (syntax: nouns repel nouns, verbs attract to nouns → natural SVO geodesics)
  - Springs / bonds (sequential adjacency, dependency trees, coref tethers — K_DEP_BOND very high)
  - Kuramoto oscillators (phase sync for binding adjectives to nouns across distance; temporal order)
  - Hopfield / associative (memory recall as energy minimization)
  - OU process + noise/annealing (creativity vs. locking; temperature as "adrenalin" or drift)
  - Negative mass + repulsion for exploration / anti-gravity "spikes"
- **Emergence**: Under right params, SVO, agreement, even complex syntax fall out as lowest-energy / geodesic paths. No rules, just physics.
- **"Naked Llama" / PhysicsLang substrate**: The LLM (or embedder + generator) is "naked" — its hidden states / next-token decisions are *steered* or *replaced* by running a physics sim on the token particles in embedding space. Logits become secondary or are projected from the final relaxed particle state.

**Historical Evolution (condensed)**:
- Early GravitationalGram: simple forces + annealing.
- PhysicsLang v1+: full particle + charge + springs + Kuramoto + splat scars + GPU acceleration (morton codes, z-order forces for N-body).
- Integration with SplatRAG: the memory *is* the splat field; generative physics runs on top of (or is the same as) the memory physics.
- Dream loops / Watcher: use TDA (betti numbers) to detect bad loops (infinite reasoning), then inject scars or temperature.

**Current SplatRag State (as of this curation)**:
- Strong foundation: SemanticGaussian (splats as Gaussians with mean/scale/rot = position + shape + "viscosity"), GpuTissue, RadianceField (physics-inspired scoring), OscillatoryNetwork + neurons (ready for Kuramoto-style temporal grammar), TDA (lophat/gudhi), perceptual phase-locked, topological memory.
- Gaps vs. full vision:
  - No explicit per-token Particle with mass+charge+springs in the generative path.
  - PhysicsKnobs are retrieval-oriented, missing the rich generative physics (dt, viscosity, force_cap, goal_strength, syntactic_*, splat_*, kuramoto coupling, etc.).
  - OscillatoryNetwork is there but not yet wired as "grammar engine" that steers token selection or binds across sequence.
  - Splat scars / negative mass propagation for online learning/conscience not fully explicit in generative loop.
  - No "physics as production rules" — generation still likely logit-driven with physics only in retrieval/memory.

**Rebuild Goal**: Make the generative engine (and memory updates) **physics-native**. The "grammar" lives in the force field and dynamics, not in prompts or decoding rules.

Use the files in this dir + the old extracted/ as source material while porting/adapting the best dynamics into:
- src/physics/ (extend gpu_engine, tissue, add particle_system or token_physics.rs)
- src/generative/ (enhance oscillatory_network to full Kuramoto + charge + spring system; simulation_controller as the "PhysicsLang stepper")
- src/config.rs (expand PhysicsKnobs massively)
- src/ (new physics_lang.rs or integrate into generative)

See `porting_map.md`, `curated_forces_and_particles.rs`, `config_knobs.md`, `splat_scar_logic.md` etc. in this dir.

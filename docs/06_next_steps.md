# Immediate Next Steps for the Rebuild (PhysicsLang / GravitationGrammar into SplatRag)

## 1. (DONE in this full CUDA pass) Fix project-wide source hygiene (pre-existing) + CUDA infra
- Added gpu-acceleration feature, src/gpu/kernels/physics_forces.cu (zorder/integrate/kuramoto/scar from history), GpuPhysicsEngine + KernelCache fixes, dispatch in physics_lang.
- Dense files still cause parse fails (embeddings etc.); stubs or ignore for check of our modules (pre-existing). cargo fmt on clean files.
- Several src/*.rs files (embeddings.rs, others) appear to be stored as extremely long single-line blobs (history export artifact).
- This causes rustc "unclosed delimiter" false positives on line 1 because of length + possible dropped chars in past pastes.
- Recommended: run a formatter + or use `cargo fmt` + manual repair of the offending files, or re-extract clean versions from git if you have history. The new files (config.rs now clean, physics_lang/*) are properly formatted.

## 2. Wire the new knobs
- `HyperParameters::load(...)` now brings in `[physics_lang]` from splat_config.toml.
- Update any place that constructs HyperParameters manually to include or derive the physics_lang field.
- In runtime code (generative simulation_controller, dream loops, regulation), start pulling `hyper.physics_lang` and passing to new physics_step.

## 3. Integrate the stepper
- Best entry points from current code:
  - `src/generative/simulation_controller.rs` + `oscillatory_network.rs` — run Kuramoto + our physics_step in the same loop. Map "neurons" <-> "PhysicsParticle" or keep parallel and cross-influence.
  - `src/bin/dream.rs` and `src/regulation/*` — call `create_and_propagate_scar` when TDA detects high b1 or low homeostasis.
  - `src/physics/gpu_engine.rs` or tissue — for large memory splat fields, the O(N) scar propagation or future z-order forces belong here.
- For generation: after (or instead of) normal decode, take recent token embeddings + current context particles, run a few physics_step iters with the query as goal, then pick next token by proximity in the relaxed positions (or use velocity/phase as bias on logits).

## 4. POS / charge bootstrap
- For real syntactic charge you need tags. Options:
  - Python side (nltk / stanza / spacy) → pass POS along with embeddings in ingest or per-generation.
  - Simple rule-based in Rust for starters (common suffixes, lists of stopwords → neg mass).
  - See `pos_to_charge` helper in the module.

## 5. Test incrementally
- The module has a `#[test] smoke...`.
- Add a bin or expand `src/bin/test_homeostasis.rs` etc. to create 10-20 fake particles from a sentence, assign charges, step 50 times, print trajectories.
- Compare "with springs+charge" vs "plain gravity" on coherence of final clusters.

## 6. GPU path (later)
- The old history had morton codes + CUDA z-order neighbor forces for scale.
- Your existing `src/gpu/` + lophat + candle is the perfect home. Start with CPU for the grammar-on-tokens (N small), GPU for the SplatRAG memory field (N large).

## 7. "Northstar" usage
- Keep the curated/ dir as living spec while you code.
- When a piece is ported and working, move or link the source back into the curated files as "implemented in src/XXX.rs:NN".

We are rebuilding the part that makes the system *feel* alive — where syntax and meaning are not prompted but *crystallized* by physics.

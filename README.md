# PhysicsLang

**Day-1 scaffold** — language tokens as particles in a force field. Experimental research crate.

Author: **Jason Van Pham**  
AI research team (build history): Shep, Echo, Lumina · tooling: Grok, Gemini, Claude

## Idea (one line)

Syntax and coherence are treated as **emergent from forces** (charge, springs, viscosity, scars) on token-particles in embedding / latent space — not as prompt rules alone.

## Status

Clean rebuild scaffold. Compiles. Smoke test for a single `physics_step`.  
Historical code and notes live under `docs/` and the paths listed in `docs/SOURCES.md`.

## Build

```bash
cd physicslang
cargo test
cargo run --example step_smoke
```

## Layout

```text
src/lib.rs          # particle, knobs, physics_step, scar helper
docs/               # northstar + porting notes from curated rebuild
docs/SOURCES.md     # where the older dumps live on disk
```

## Not in this crate (yet)

Full SplatRag monorepo, CUDA force kernels, embedder wiring, generation loop.

## License

MIT OR Apache-2.0 for this scaffold. Historical dumps retain their original project licensing.

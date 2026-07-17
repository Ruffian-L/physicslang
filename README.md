# PhysicsLang

**Day-1 scaffold** — language tokens as particles in a force field. Experimental research crate.

Author: **Jason Van Pham**  
AI research team (build history): Shep, Echo, Lumina · tooling: Grok, Gemini, Claude

## Idea (one line)

Syntax and coherence are treated as **emergent from forces** (charge, springs, viscosity, scars) on token-particles in embedding / latent space — not as prompt rules alone.

**Ontological inversion** is first-class: negative steering along a concept axis maps a state toward its **structured antipode** (Householder reflection), not to zero. See `antipode` / `invert_toward_antipode` / `StepContext::inversion_axis`.

## Status

Day-1+ scaffold. Compiles. Unit tests + smoke examples.  
Historical notes under `docs/` and `docs/SOURCES.md`.

## Build

```bash
cd physicslang
cargo test
cargo run --example step_smoke
cargo run --example inversion_smoke
```

## Layout

```text
src/lib.rs                 # particles, forces, scars, inversion
examples/step_smoke.rs
examples/inversion_smoke.rs
docs/                      # northstar + porting notes
docs/SOURCES.md
```

## Not in this crate (yet)

Full SplatRag monorepo, CUDA force kernels, embedder wiring, generation loop.

## License

MIT OR Apache-2.0 for this scaffold. Historical dumps retain their original project licensing.

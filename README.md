# PhysicsLang

**Day-1 scaffold** — language tokens as particles in a force field. Experimental research crate.

Author: **Jason Van Pham**  
AI research team (build history): Shep, Echo, Lumina · tooling: Grok, Gemini, Claude

## Idea (one line)

Syntax and coherence are treated as **emergent from forces** (charge, springs, viscosity, scars) on token-particles in embedding / latent space — not as prompt rules alone.

**Ontological inversion** is first-class: negative steering along a concept axis maps a state toward its **structured antipode** (Householder reflection), not to zero. See `antipode` / `invert_toward_antipode` / `StepContext::inversion_axis`.

### Human language vs latent language

| Mode | Particles are | Charge / POS | Example |
|------|----------------|--------------|---------|
| Human-facing | Word/phrase embeds | Optional role bias | `qwen_particles`, `qwen_ab_charge` |
| **Latent language** | Points in **64D forge** (hidden/latent states) | **Off** — geometry + springs + goal + scar + inversion | `latent_lang` |

Same engine. Latent mode is PhysicsLang where the “utterance” is a trajectory in latent space, not an English sentence.

## Status

Day-1+ scaffold. Compiles. Unit tests + smoke examples.  
Historical notes under `docs/` and `docs/SOURCES.md`.

## Build

```bash
cd physicslang
cargo test
cargo run --example step_smoke
cargo run --example inversion_smoke
cargo run --example forge_64d

# needs Qwen embed server (llama-server --embeddings on :8302)
cargo run --example qwen_particles
cargo run --example qwen_ab_charge
cargo run --example latent_lang
```

Embed default: `Qwen3-Embedding-8B-Q8_0.gguf` → 4096-d → bin-average project to **64-d** unit vectors → physics step.

## Layout

```text
src/lib.rs                 # particles, forces, scars, inversion, 64D helpers
examples/step_smoke.rs
examples/inversion_smoke.rs
examples/forge_64d.rs      # 64D cloud + scar + inversion (latent crucible)
docs/                      # northstar + porting notes
docs/SOURCES.md
```

## Not in this crate (yet)

Full SplatRag monorepo, CUDA force kernels, embedder wiring, generation loop.

## License

MIT OR Apache-2.0 for this scaffold. Historical dumps retain their original project licensing.

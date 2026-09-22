# polyvocoder (Rust port)

> **Rust port of the Polyvocoder universal head.**
> Schema-parity with the Python and TypeScript implementations.
> Part of the Quilt substrate walker polyformalism fleet.

## Quick Start

```rust
use polyvocoder::{canary_hash, generate_dials};

fn main() {
    let h = canary_hash();
    assert_eq!(h, 0x024a555471370b18d);

    let dials = generate_dials(70051917);
    // dials[0..7] are signed int16 mirrors
    // dials[8..15] are zeros
    println!("Canary hash: 0x{:016x}", h);
}
```

## Build

```bash
cargo build --release
cargo test
cargo run --release --bin canary  # verify canary
cargo run --release --bin polyvocoder  # demo
```

## Fleet Canary

The Rust port pins to the Quilt fleet canary:

**`fnv1a-64('café Δ 日本語') = 0x024a555471370b18d`**

This is verified by the `canary` binary and by the `test_canary` unit test.

## Schema Parity

Rust types mirror Python dataclasses with bidirectional JSON field naming:

| Rust | Python | JSON wire |
|---|---|---|
| `JEVFeatures` | `JEVFeatures` | `{canon_worthy, distinct_voice, ...}` |
| `DecodedSample` | `DecodedSample` | `{latent, text, image, audio}` |
| `PipelineResult` | `PipelineResult` | `{features, decoded_samples}` |

JSON serialization uses both snake_case and camelCase aliases, so the wire
format is interoperable with the TypeScript binding.

## Why Rust?

The Rust port provides:
- **Algorithmic efficiency** — FNV-1a computation in <1ns
- **Memory safety** — no unsafe code in this crate
- **No-allocation dials** — stack-allocated `[i16; 16]` array
- **`no_std` compatible** — could ship as WASM or embedded firmware
- **Cross-platform** — works on Linux, macOS, Windows, BSD

## Use Cases

- **Browser WASM module**: compile to wasm32-unknown-unknown, ship as ~5KB module
- **Embedded firmware**: power a microcontroller to compute the canary hash
- **Edge computing**: run polyvocoder on a Raspberry Pi without Python
- **Performance-critical paths**: avoid GIL and memory allocation

## Tests

```bash
cargo test
# running 5 tests
# test test_canary ... ok
# test test_dials ... ok
# test test_fnv1a_empty ... ok
# test test_fnv1a_one_byte ... ok
# test test_features_serde ... ok
```

## Polyformalism Fleet

This is port 7 of the polyformalism fleet:
1. Python (SuperInstance/polyvocoder)
2. TypeScript (SuperInstance/polyvocoder-bindings)
3. Rust (SuperInstance/polyvocoder-rust) ← you are here
4. Bash (in /workspace/research/scripts)
5. JavaScript ESM (in @superinstance/polyvocoder-bindings)
6. C#/.NET 9 (planned)
7. **Rust** (this crate)

All ports share the canary `0x024a555471370b18d`.

## License

MIT — Casey / SuperInstance, Sept 22, 2026

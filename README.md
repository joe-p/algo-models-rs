# Algorand-Rust-FFIs

This is a cargo workspace for Rust implementations of core Algorand functionality that is exposed to other languages via FFIs.

## Crates

- [algo_models](./crates/algo_models) - Currently a spike to determine feasibility of using UniFFI and wasm-pack. Handles msgpack encoding and decoding of Algorand transactions and allows attaching signatures to transactions.

## ADRs

[./docs/decisions](./docs/decisions) contains the ADRs for this project. Specific implementations might have their own ADRs in their respective directories.

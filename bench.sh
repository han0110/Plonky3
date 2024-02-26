#!/bin/bash

for example in "prove_m31_keccak" "prove_m31_poseidon2" "prove_baby_bear_keccak" "prove_baby_bear_poseidon2" "prove_goldilocks_keccak" "prove_goldilocks_poseidon"; do
    RUST_LOG=warn cargo run --release --features parallel --example "$example" > "$example.log"
    # RUST_LOG=warn RAYON_NUM_THREADS=1 cargo run --release --features parallel --example "$example" > "$example.log"
    # RUST_LOG=warn RAYON_NUM_THREADS=6 cargo run --release --features parallel --example "$example" >> "$example.log"
    # RUST_LOG=warn RAYON_NUM_THREADS=12 cargo run --release --features parallel --example "$example" >> "$example.log"
    # RUST_LOG=warn RAYON_NUM_THREADS=18 cargo run --release --features parallel --example "$example" >> "$example.log"
    # RUST_LOG=warn RAYON_NUM_THREADS=24 cargo run --release --features parallel --example "$example" >> "$example.log"
done

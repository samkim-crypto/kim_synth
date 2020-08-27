## THIS IS PROTOTYPE CODE FOR RESEARCH - NOT SUITABLE OR TESTED FOR PRODUCTION USAGE

This software is for performance testing ONLY! It may have security
vulnerabilities that could be exploited in any read-world deployment.

# Kim synthesizer scheme

This repo contains a prototype implementation of the Kim pseudorandom
synthesizer scheme. Pseudorandom synthesizers can be used as a building block
for a variety of cryptographic primitives including pseudorandom functions
(blockciphers) and pseudorandom generators. The security of the Kim Synthesizer
is based on hard problems on lattices. 

For the accompanying research paper, see: https://eprint.iacr.org/2020/233.pdf

## Installation & Usage

Requires Rust - https://rustup.rs/

Once installed, you can download with:

```bash
git clone https://github.com/samkim-crypto/kim_synth/
cd recrypt
cargo build
# Optional: builds and opens documentation
cargo doc --no-deps --open
```

By default, running with `cargo run` gives the benchmarks. Ensure to run with
`cargo run --release` to get profiles for optimised code.

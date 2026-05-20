# 18fa3 Regression Prover

Generates rogue SP1 PLONK and Groth16 proofs that demonstrate audit finding
18fa3: pi2 (exit_code) and pi3 (vk_root) are not pinned in the proof-conversion
o1js circuits.

The attack forges sp1_vk_digest (pi0) to the legitimate noriHeliosProgramPi0
value while using a rogue vk_root (pi3), proving an attacker can produce a
proof that the unpatched proof-conversion pipeline accepts.

## Structure

- `guest/` -- trivial SP1 guest that commits attacker-chosen bytes
- `host/` -- host binary that generates PLONK/Groth16 proofs via CPU prover
- `host/src/bin/bn254_to_koalabear.rs` -- utility to decompose a BN254 scalar
  into 8 KoalaBear field elements (u31 limbs, big-endian)
- `host/bin/bn254_to_koalabear` -- precompiled binary of the above
- `sp1/` -- SP1 v6.1.0 source (local clone, patched for the exploit)

## SP1 patches (applied on top of base)

1. `sp1/crates/recursion/circuit/src/machine/core.rs` -- forge sp1_vk_digest
   to noriHeliosProgramPi0 KoalaBear decomposition instead of hash(VK)
2. `sp1/crates/prover/src/worker/prover/recursion.rs` -- set
   `vk_verification: false` so the wrap circuit skips vk_root validation

## noriHeliosProgramPi0 KoalaBear decomposition

Source (AUDIT branch):
https://github.com/Nori-zk/nori-bridge-head/blob/AUDIT/nori-elf/nori-sp1-helios-program.pi0.json

BN254 scalar: 369200071524637939057719045636416842297844707735373281048784868601844026256

KoalaBear decomposition (8 x u31):

    digest[0] = 1752882145  (0x687adbe1)
    digest[1] = 968045619   (0x39b33433)
    digest[2] = 1380945032  (0x524f8c88)
    digest[3] = 1794649364  (0x6af82d14)
    digest[4] = 1462922488  (0x57326cf8)
    digest[5] = 2019664520  (0x7861a288)
    digest[6] = 1179048077  (0x4646d88d)
    digest[7] = 1839549328  (0x6da54b90)

Reproduced with:

    host/bin/bn254_to_koalabear
    # or: rustc host/src/bin/bn254_to_koalabear.rs -o bn254_to_koalabear && ./bn254_to_koalabear

## Generating rogue proofs

Requires a machine with sufficient resources (tested on 96 cores / 256 GB RAM).

    cargo run --release -- plonk output    # PLONK proof
    cargo run --release -- groth16 output  # Groth16 proof

Output JSON files are saved to the specified directory in the same format as
nori-bridge-head (serde_json on SP1ProofWithPublicValues).

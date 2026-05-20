#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Trivial program: read attacker-chosen bytes and commit them as public output.
    // This gives the attacker full control over committed_values_digest (pi1).
    let payload: Vec<u8> = sp1_zkvm::io::read_vec();
    sp1_zkvm::io::commit_slice(&payload);
}

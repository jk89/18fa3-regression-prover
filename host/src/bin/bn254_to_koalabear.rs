use std::env;

// noriHeliosProgramPi0 at audit time (AUDIT branch):
// https://github.com/Nori-zk/nori-bridge-head/blob/AUDIT/nori-elf/nori-sp1-helios-program.pi0.json
const NORI_HELIOS_PI0: &str = "369200071524637939057719045636416842297844707735373281048784868601844026256";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 {
        args[1].clone()
    } else {
        println!("No argument provided, using noriHeliosProgramPi0 from AUDIT branch.");
        NORI_HELIOS_PI0.to_string()
    };

    // Simple big-integer: parse decimal string, divmod by 2^31 eight times.
    let shift = 1u64 << 31;
    let mut digits: Vec<u8> = input.bytes().map(|b| b - b'0').collect();

    let mut words = Vec::new();
    for _ in 0..8 {
        let mut remainder: u64 = 0;
        let mut quotient = Vec::new();
        for &d in &digits {
            let cur = remainder * 10 + d as u64;
            quotient.push((cur / shift) as u8);
            remainder = cur % shift;
        }
        words.push(remainder as u32);
        // Remove leading zeros from quotient.
        while quotient.len() > 1 && quotient[0] == 0 {
            quotient.remove(0);
        }
        digits = quotient;
    }
    words.reverse();

    println!("BN254 scalar: {}", input);
    println!("KoalaBear decomposition (8 x u31):");
    for (i, w) in words.iter().enumerate() {
        println!("  digest[{}] = {}  (0x{:08x})", i, w, w);
    }

    // Emit as Rust array literal.
    let rust_vals: Vec<String> = words
        .iter()
        .map(|w| format!("SP1Field::from_canonical_u32({})", w))
        .collect();
    println!("\nRust literal:");
    println!("  [{}]", rust_vals.join(", "));
}

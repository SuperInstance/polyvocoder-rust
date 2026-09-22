use polyvocoder::{generate_dials, canary_hash};

fn main() {
    println!("Polyvocoder Rust port");
    println!("====================\n");
    println!("Canary: 0x{:016x}", canary_hash());

    let seed: u64 = 70051917;
    let dials = generate_dials(seed);
    println!("Dials for seed {}:", seed);
    for (i, d) in dials.iter().enumerate() {
        println!("  [{}] = {}", i, d);
    }
}

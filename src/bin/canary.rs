use polyvocoder::{canary_hash, verify_canary};

fn main() {
    let h = canary_hash();
    let verified = verify_canary();
    println!("fnv1a-64('café Δ 日本語') = 0x{:016x}", h);
    println!("verify: {}", if verified { "PASS" } else { "FAIL" });
    std::process::exit(if verified { 0 } else { 1 });
}

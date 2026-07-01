fn main() {
    let digest = sealyn::sha256(b"hello, evidence");
    println!("sealyn v{}", env!("CARGO_PKG_VERSION"));
    println!("sha256(\"hello, evidence\") = {}", hex::encode(digest));
}

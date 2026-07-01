use sealyn::{sha256, verify, Signer, SoftwareSigner};

fn main() {
    let signer = SoftwareSigner::new("sw-p256-0001");

    let evidence = b"hello, evidence";
    let digest = sha256(evidence);
    let signature = signer.sign(&digest).expect("signing failed");
    let ok = verify(&signer.public_key(), &digest, &signature);

    println!("Sealyn v{}", env!("CARGO_PKG_VERSION"));
    println!("signer id : {}", signer.signer_id());
    println!("digest    : {}", hex::encode(digest));
    println!("signature : {}", hex::encode(&signature));
    println!("verified  : {}", ok);
}

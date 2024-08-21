use solana_sdk::signer::{keypair::Keypair, Signer};

pub fn generate_keypair() {
    let keypair = Keypair::new();

    println!("The public key is: {}", keypair.pubkey());
    println!("The secret key is: {:?}", keypair.to_bytes());
    println!("✅ Finished!");
}

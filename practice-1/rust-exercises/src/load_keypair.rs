use dotenv::dotenv;
use serde_json;
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::env;

pub fn load_keypair() {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let keypair = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    println!("Public key: {}", keypair.pubkey());
}

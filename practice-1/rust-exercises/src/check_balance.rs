use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn check_balance() {
    let connection = RpcClient::new("https://api.devnet.solana.com");
    println!("⚡ Connected to devnet");

    let public_key = Pubkey::from_str("GCWUHj5LqLvMH4bx3KoSG978UmNX5N78usyXMg43JcMS")
        .expect("Invalid public key");
    let balance_in_lamports = connection
        .get_balance(&public_key)
        .expect("Failed to get balance");

    let balance_in_sol = balance_in_lamports as f64 / LAMPORTS_PER_SOL as f64;

    println!(
        "💰 The balance for the wallet at address {} is: {:.9} SOL",
        public_key, balance_in_sol
    );
}

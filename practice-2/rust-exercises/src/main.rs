mod send_sol;
mod create_token_mint;
mod create_token_account;
mod mint_tokens;
mod create_token_metadata;

fn main() {
    send_sol::send_sol();
    println!();
    create_token_mint::create_token_mint();
    println!();
    create_token_account::create_token_account();
    println!();
    mint_tokens::mint_tokens();
    println!();
    create_token_metadata::create_token_metadata();
}

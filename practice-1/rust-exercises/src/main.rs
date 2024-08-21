mod generate_keypair;
mod load_keypair;
mod check_balance;

fn main() {
    generate_keypair::generate_keypair();
    println!();
    load_keypair::load_keypair();
    println!();
    check_balance::check_balance();
}

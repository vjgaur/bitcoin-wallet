use std::env;

fn main() {
    dotenv::from_filename(".env").ok();
    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
}

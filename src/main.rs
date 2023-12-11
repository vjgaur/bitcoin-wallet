use bdk::{bitcoin::Network, database::MemoryDatabase, wallet, Wallet};
use std::env;
fn main() -> anyhow::Result<()> {
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();
    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);
    let wallet = Wallet::new(
        descriptor,
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    Ok(())
}

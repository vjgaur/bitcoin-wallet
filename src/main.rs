use std::env;

fn main() -> anyhow::Result<()> {
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();
    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();
    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);
    Ok(())
}

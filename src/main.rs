use dotenv;
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};

mod eth;
mod near;
mod utils;

#[derive(Serialize, Deserialize)]
pub struct Bloom {
    #[serde(with = "BigArray")]
    logs: [u8; 256],
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    Ok(())
}

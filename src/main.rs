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

    let client = near::setup_client("");
    let _ = near::insert_filter(&client).await;

    Ok(())
}

use std::env;
use std::{thread, time};

use dotenv;

mod eth;
mod near;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let eth_transport = eth::setup_transport(&env::var("WS_ENDPOINT").unwrap()).await;
    let near_client = near::setup_client(&env::var("RPC_ENDPOINT").unwrap());
    let duration = time::Duration::from_secs(17);
    let (mut block_number, mut logs) = eth::get_init_block(&eth_transport).await;

    loop {
        near::insert_filter(&near_client, block_number.0[0], utils::Bloom { logs })
            .await
            .expect("Inseter error");
        eth::get_filter_and_block_number(&eth_transport, &mut block_number, &mut logs).await;
        block_number.0[0] += 1;
        thread::sleep(duration);
    }
}

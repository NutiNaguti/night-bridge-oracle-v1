use std::env;
use std::{thread, time};

use dotenv;
use near_jsonrpc_client::JsonRpcClient;
use web3::transports::WebSocket;
use web3::types::U64;

mod eth;
mod near;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let eth_transport = eth::setup_transport(&env::var("WS_ENDPOINT").unwrap()).await;
    let near_client = near::setup_client(&env::var("RPC_ENDPOINT").unwrap());
    // let duration = time::Duration::from_secs(17);
    // let (mut block_number, mut logs) = eth::get_init_block(&eth_transport).await;

    // loop {
    //     near::insert_filter(&near_client, block_number.0[0], utils::Bloom { logs })
    //         .await
    //         .expect("Inseter error");
    //     eth::get_filter_and_block_number(&eth_transport, &mut block_number, &mut logs).await;
    //     block_number.0[0] += 1;
    //     thread::sleep(duration);
    // }
    test_insert_filter(&eth_transport, &near_client).await;
}

async fn test_insert_filter(eth_transport: &WebSocket, near_client: &JsonRpcClient) {
    let mut block_number = U64::from_dec_str("8209810").unwrap();
    let mut logs: [u8; 256] = [0; 256];
    eth::get_filter_and_block_number(eth_transport, &mut block_number, &mut logs).await;
    near::insert_filter(&near_client, block_number.0[0], utils::Bloom { logs })
        .await
        .expect("Inseter error");
}

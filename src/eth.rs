use hex_literal::hex;
use std::{env, str::FromStr};

use web3::{
    ethabi::ethereum_types::BloomInput,
    signing,
    transports::WebSocket,
    types::{BlockId, BlockNumber, H160, U64},
};

pub async fn setup_transport() -> WebSocket {
    WebSocket::new(&env::var("WS_ENDPOINT").unwrap())
        .await
        .unwrap()
}

pub async fn get_filter_and_block_number(transport: WebSocket) -> (u64, [u8; 256]) {
    let web3 = web3::Web3::new(transport);
    let block_number = web3.eth().block_number().await.unwrap();

    let block = web3
        .eth()
        .block(BlockId::Number(BlockNumber::Latest))
        .await
        .unwrap();
    let logs_bloom = block.unwrap().logs_bloom.unwrap();
    (block_number.0[0], logs_bloom.0)
}

pub async fn validate(transport: WebSocket) -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();
    // mock topic
    let topic_1 = hex!("3b874d464775b5082b95c98fb5f815494cc129e32c4e8a07a0bb98e710f8c25c");

    let block_number = &args[1];
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));

    let contract_address = &args[2];
    let normalized_address = H160::from_str(contract_address).unwrap();

    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block(block_number).await?;
    let logs_bloom = block.unwrap().logs_bloom.unwrap();

    let is_valid = logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        &signing::keccak256("Locked(bytes32)".as_bytes()),
    ))) & logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        normalized_address.as_bytes(),
    ))) & logs_bloom.contains_input(BloomInput::Raw(&topic_1));

    println!("Validated: {}", is_valid);

    Ok(())
}

use hex_literal::hex;
use std::{env, str::FromStr};

use web3::{
    ethabi::ethereum_types::BloomInput,
    signing,
    transports::WebSocket,
    types::{BlockId, BlockNumber, H160, U64},
};

pub async fn setup_transport(ws_endpoint: &String) -> WebSocket {
    WebSocket::new(&ws_endpoint).await.unwrap()
}

pub async fn get_init_block(transport: &WebSocket) -> (U64, [u8; 256]) {
    let web3 = web3::Web3::new(transport);
    let block_number = web3.eth().block_number().await.unwrap();
    let block = web3.eth().block(BlockId::from(block_number)).await.unwrap();
    let logs_bloom = block.unwrap().logs_bloom.unwrap();
    (block_number, logs_bloom.0)
}

pub async fn get_filter_and_block_number(
    transport: &WebSocket,
    block_number: &mut U64,
    logs: &mut [u8; 256],
) {
    let web3 = web3::Web3::new(transport);
    let block = web3
        .eth()
        .block(BlockId::from(block_number.to_owned()))
        .await
        .unwrap();
    *logs = block.unwrap().logs_bloom.unwrap().0;
}

pub async fn validate(transport: &WebSocket) -> web3::Result<()> {
    // mocked data
    let topic_1 = hex!("3b874d464775b5082b95c98fb5f815494cc129e32c4e8a07a0bb98e710f8c25c");

    let block_number = "8029981";
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));

    let contract_address = "0x9431f9bba577B037D97ad6F7086a00eFB572c871";
    let normalized_address = H160::from_str(contract_address).unwrap();

    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block(block_number).await?;
    let logs_bloom = block.unwrap().logs_bloom.unwrap();

    let is_valid = logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        &signing::keccak256("Locked(bytes32)".as_bytes()),
    ))) & logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        normalized_address.as_bytes(),
    ))) & logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(&topic_1)));

    println!("Validated: {}", is_valid);

    Ok(())
}

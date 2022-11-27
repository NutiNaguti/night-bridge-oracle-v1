use hex::FromHex;
use hex_literal::hex;
use std::env;
use std::i64;
use std::str::FromStr;
use web3::ethabi::ethereum_types::BloomInput;
use web3::futures::{future, StreamExt};
use web3::transports::WebSocket;
use web3::types::{BlockId, BlockNumber, FilterBuilder, H160, H2048, U64};
use web3::{signing, Web3};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();
    // mock topic
    let topic_1 = hex!("3b874d464775b5082b95c98fb5f815494cc129e32c4e8a07a0bb98e710f8c25c");

    let block_number = &args[1];
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));

    let contract_address = &args[2];
    let normalized_address = H160::from_str(contract_address).unwrap();

    let transport = web3::transports::WebSocket::new(
        // ws for testnet
        "wss://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )
    .await?;
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

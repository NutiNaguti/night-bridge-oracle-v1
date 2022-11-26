use hex_literal::hex;
use std::env;
use std::str::FromStr;
use web3::ethabi::ethereum_types::BloomInput;
use web3::futures::{future, StreamExt};
use web3::transports::WebSocket;
use web3::types::{BlockId, BlockNumber, FilterBuilder, H160, H2048, U64};
use web3::{signing, Web3};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();

    let block_number = &args[1];
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));

    let contract_address = &args[2];
    let normalized_address = H160::from_str(contract_address).unwrap();

    let event_data_1 = 8023385;
    let event_data_2 = H160::from_str("0x8cab5e96e1ab09e8678a8ffc75b5d818e73d4707").unwrap();
    let event_data_3 = 12;

    let transport = web3::transports::WebSocket::new(
        "wss://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )
    .await?;
    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block(block_number).await?;
    let logs_bloom = block.unwrap().logs_bloom.unwrap();

    println!("logs filter: {:?}", logs_bloom);

    let contains = logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        &signing::keccak256("Locked3(uint256,address,uint256)".as_bytes()),
    )));

    println!("Event signature: {}", contains);

    let contains = logs_bloom.contains_input(BloomInput::Hash(&signing::keccak256(
        normalized_address.as_bytes(),
    )));

    println!("Contract address: {}", contains);

    // If we join data step by step then we are getting too big hex number
    // I think this is wrong algorithm
    // Need more research
    let contains = logs_bloom.contains_input(BloomInput::Raw(&signing::keccak256(
        &signing::keccak256(H2048::from_str("").unwrap().as_bytes()),
    )));

    println!("Event data: {}", contains);

    Ok(())
}

// Not used but may be useful
async fn eth_subscribe(web3: Web3<WebSocket>, normalized_address: H160) -> web3::Result<()> {
    let filter = FilterBuilder::default()
        .address(vec![normalized_address])
        .topics(
            Some(vec![hex!(
                "55016b6cc60c3d15b7b1ebd0ab766c07b3082c98a5b4d1d7ff012a97652a4b1d"
            )
            .into()]),
            None,
            None,
            None,
        )
        .build();

    let sub = web3.eth_subscribe().subscribe_logs(filter).await?;

    sub.for_each(|log| {
        match log {
            Ok(data) => {
                println!("{:?}", data)
            }
            Err(error) => panic!("{:?}", error),
        }
        future::ready(())
    })
    .await;

    Ok(())
}

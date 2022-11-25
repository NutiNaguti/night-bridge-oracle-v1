use hex_literal::hex;
use std::env;
use std::str::FromStr;
use web3::futures::{future, StreamExt};
use web3::types::{FilterBuilder, H160};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();

    // let block_number = &args[1];
    // let block_number = BlockId::Number(BlockNumber::Number(
    //     U64::from_dec_str(&block_number).unwrap(),
    // ));

    let contract_address = &args[1];
    let normalized_address: H160 = H160::from_str(contract_address).unwrap();

    let transport = web3::transports::WebSocket::new(
        "wss://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )
    .await?;
    let web3 = web3::Web3::new(transport);

    // let block = web3.eth().block(block_number).await?;
    // let logs_bloom = block.unwrap().logs_bloom.unwrap();
    // println!("logs filter: {}", logs_bloom);

    // TODO
    // let encoded_address = signing::keccak256(contract_address.as_bytes());
    // let encoded_event_signature = signing::keccak256(&signing::keccak256(
    //     "Locked(uint256,address,uint256)".as_bytes(),
    // ));
    // let encoded_logged_data = signing::keccak256(&signing::keccak256(logged_data.as_bytes()));
    // Filter for Hello event in our contract

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

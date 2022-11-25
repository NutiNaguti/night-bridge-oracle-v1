use std::env;
use web3::types::BlockId;
use web3::types::BlockNumber;
use web3::types::U64;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();
    let block_number = &args[1];
    let block_number = BlockId::Number(BlockNumber::Number(
        U64::from_dec_str(&block_number).unwrap(),
    ));
    println!("block number: {:?}", block_number);

    let transport = web3::transports::Http::new(
        "https://eth-goerli.g.alchemy.com/v2/fq5FsW3IggL1giodxIhWRqW-er0MpDbi",
    )?;
    let web3 = web3::Web3::new(transport);

    let block = web3.eth().block(block_number).await?;
    match block {
        Some(data) => println!("logs bloom: {:?}", data.logs_bloom),
        _ => {}
    }

    Ok(())
}

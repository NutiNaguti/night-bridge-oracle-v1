use std::io::{self, Write};

use near_primitives::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Bloom {
    pub logs: [u8; 256],
}

#[derive(Serialize, Deserialize)]
pub struct BloomRequest {
    pub block_number: u64,
    // base64
    pub logs: String,
}

pub fn input(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

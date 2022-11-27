use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};

#[derive(Serialize, Deserialize)]
pub struct Bloom {
    #[serde(with = "BigArray")]
    logs: [u8; 256],
}

pub fn input(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

#[macro_use] extern crate error_chain;

#[allow(deprecated)]
mod error;
mod stats;

use std::io::Read;
use std::fs::File;

use crate::error::Result;
use crate::stats::Stats;

fn main() -> Result<()> {
    let mut file = File::open("weights/hunter.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let hunter_weights: Stats = serde_json::from_str(&contents)?;
    println!("{:?}", hunter_weights);
    Ok(())
}

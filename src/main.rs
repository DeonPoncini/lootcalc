#[macro_use] extern crate error_chain;

#[allow(deprecated)]
mod error;
mod stats;
mod restriction;

use std::io::Read;
use std::fs::File;

use crate::error::Result;
use crate::stats::Stats;
use crate::restriction::Restriction;

fn load_weights(class: &str, spec: &str) -> Result<Stats> {
    let mut file = File::open(format!("weights/{}_{}.json", class, spec))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let stats: Stats = serde_json::from_str(&contents)?;
    Ok(stats)
}

fn load_restrictions(class: &str) -> Result<Restriction> {
    let mut file = File::open(format!("restrict/{}.json", class))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let restrict: Restriction = serde_json::from_str(&contents)?;
    Ok(restrict)
}

fn process(class: &str, spec: &str) -> Result<()> {
    let weights = load_weights(class, spec)?;
    let restrict = load_restrictions(class)?;
    println!("{:?}", weights);
    println!("{:?}", restrict);
    Ok(())
}

fn main() -> Result<()> {
    process("hunter", "dps")?;
    Ok(())
}

#[macro_use] extern crate error_chain;

#[allow(deprecated)]
mod error;
mod gear;
mod score;
mod stats;
mod restriction;

use std::collections::BinaryHeap;
use std::io::Read;
use std::fs::File;

use crate::error::Result;
use crate::gear::Gears;
use crate::score::Score;
use crate::stats::Stats;
use crate::restriction::Restriction;

// how many generations of gear we support
const MAX_SEQUENCES: usize = 5;

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

fn load_gear(slot: &str) -> Result<Gears> {
    let mut file = File::open(format!("gear/{}.json", slot))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let gear: Gears = serde_json::from_str(&contents)?;
    Ok(gear)
}

fn process(class: &str, spec: &str, gear: &[Gears])
                   -> Result<Vec<BinaryHeap<Score>>> {
    let weights = load_weights(class, spec)?;
    let restrict = load_restrictions(class)?;

    let mut output = Vec::new();
    for _ in 0..MAX_SEQUENCES {
        output.push(BinaryHeap::new());
    }

    for gg in gear {
        for g in &gg.gear {
            let score = score::calculate_score(class, &weights, &restrict, &g);
            if g.sequence >= MAX_SEQUENCES {
                println!("Cannot process {} due to invalid sequence {}",
                         g.name, g.sequence);
                continue;
            }
            // store in the heap
            output[g.sequence].push(score);
        }
    }
    // now modify all values to calculate the offsets

    Ok(output)
}

fn main() -> Result<()> {
    let mut gear = Vec::new();
    // load up the gear
    gear.push(load_gear("hands")?);

    // process it per class and spec
    let results = process("hunter", "dps", &gear)?;
    let mut x = 0;
    for r in results {
        println!("Sequence {}", x);
        for s in r {
            println!("{}\t{}", s.name(), s.score());
        }
        x = x + 1;
    }
    Ok(())
}

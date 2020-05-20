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
const MAX_GENERATION: usize = 5;
const GENERATIONS: [&'static str; MAX_GENERATION] = [
    "Pre-raid",
    "Molten Core",
    "Blackwing Lair",
    "Temple of Ahn'Qiraj",
    "Naxxramas"
];

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

    let mut bases = Vec::new();
    let mut output = Vec::new();
    for _ in 0..MAX_GENERATION {
        bases.push(BinaryHeap::new());
        output.push(BinaryHeap::new());
    }

    for gg in gear {
        for g in &gg.gear {
            let score = score::calculate_score(class, &weights, &restrict, &g);
            if g.generation >= MAX_GENERATION {
                println!("Cannot process {} due to invalid generation {}",
                         g.name, g.generation);
                continue;
            }
            // store in the heap
            bases[g.generation].push(score);
        }
    }

    // convert this to a sorted vector
    let mut intermediate = Vec::new();
    let b2 = bases.clone();
    for b in b2 {
        intermediate.push(b.into_sorted_vec());
    }

    // now modify all values to calculate the offsets
    for x in 0..MAX_GENERATION {
        for y in 0..intermediate[x].len() {
            // the upgrade cost. This is the difference between the score and the
            // maximum value from the previous generation. If there is no values in
            // previous generation, upgrade cost is 0
            let upgrade;
            if x == 0 { upgrade = 0; }
            else {
                match bases[x-1].peek() {
                    Some(b) => {
                        upgrade = intermediate[x][y].score() - b.score();
                    }
                    None => upgrade = 0,
                }
            }

            // the replacement cost is the difference to the next larger value in
            // the current generation, or the BIS in the next generation if this
            // value is BIS. If its max generation, upgrade cost is score
            let replacement;
            if x == MAX_GENERATION-1 {
                replacement = intermediate[x][y].score();
            } else {
                if y == intermediate[x].len()-1 {
                    match bases[x+1].peek() {
                        Some(b) => {
                            replacement = b.score() - intermediate[x][y].score();
                        }
                        None => replacement = intermediate[x][y].score(),
                    }
                } else {
                    replacement = intermediate[x][y+1].score() -
                        intermediate[x][y].score();
                }
            }
            intermediate[x][y].set_upgrade(upgrade as f32);
            intermediate[x][y].set_replacement(replacement as f32);
        }
    }

    // convert this back into a binary heap to reorder
    for x in 0..intermediate.len() {
        for y in &intermediate[x] {
            output[x].push(y.clone());
        }
    }

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
        println!("{}", GENERATIONS[x]);
        for s in r {
            println!("{}\t{}", s.name(), s.score());
        }
        x = x + 1;
    }
    Ok(())
}

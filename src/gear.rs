use serde_derive::{Deserialize, Serialize};

use crate::stats::Stats;

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    name: String,
    sequence: u32,
    material: String,
    weapon: String,
    restrict: String,
    stats: Stats
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gears {
    gear: Vec<Gear>,
}

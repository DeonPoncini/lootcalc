use serde_derive::{Deserialize, Serialize};

use crate::stats::Stats;

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    pub name: String,
    pub sequence: usize,
    pub material: String,
    pub weapon: String,
    pub restrict: String,
    pub stats: Stats
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gears {
    pub gear: Vec<Gear>,
}

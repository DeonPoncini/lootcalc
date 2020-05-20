use serde_derive::{Deserialize, Serialize};

use crate::stats::Stats;

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    pub name: String,
    pub generation: usize,
    #[serde(default)]
    pub material: String,
    #[serde(default)]
    pub weapon: String,
    #[serde(default)]
    pub restrict: String,
    pub stats: Stats
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gears {
    pub gear: Vec<Gear>,
}

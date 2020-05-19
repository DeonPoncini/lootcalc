use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Restriction {
    pub armor: Vec<String>,
    pub weapons: Vec<String>,
}

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Stats {
    #[serde(default)]
    pub attributes: Attributes,
    #[serde(default)]
    pub spell: Spell,
    #[serde(default)]
    pub melee: Melee,
    #[serde(default)]
    pub ranged: Ranged,
    #[serde(default)]
    pub weaponskill: Weaponskill,
    #[serde(default)]
    pub defense: Defense,
    #[serde(default)]
    pub resistances: Resistances,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Attributes {
    #[serde(default)]
    pub strength: f32,
    #[serde(default)]
    pub agility: f32,
    #[serde(default)]
    pub stamina: f32,
    #[serde(default)]
    pub intellect: f32,
    #[serde(default)]
    pub spirit: f32,
    #[serde(default)]
    pub health: f32,
    #[serde(default)]
    pub mana: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Spell {
    #[serde(default)]
    pub spell_damage: f32,
    #[serde(default)]
    pub healing: f32,
    #[serde(default)]
    pub spell_hit: f32,
    #[serde(default)]
    pub spell_crit: f32,
    #[serde(default)]
    pub spell_pen: f32,
    #[serde(default)]
    pub mp5: f32,
    #[serde(default)]
    pub arcane: f32,
    #[serde(default)]
    pub fire: f32,
    #[serde(default)]
    pub frost: f32,
    #[serde(default)]
    pub nature: f32,
    #[serde(default)]
    pub shadow: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Melee {
    #[serde(default)]
    pub ap: f32,
    #[serde(default)]
    pub feral_ap: f32,
    #[serde(default)]
    pub weapon_dps: f32,
    #[serde(default)]
    pub weapon_speed: f32,
    #[serde(default)]
    pub hit: f32,
    #[serde(default)]
    pub crit: f32,
    #[serde(default)]
    pub haste: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Ranged {
    #[serde(default)]
    pub ranged_dps: f32,
    #[serde(default)]
    pub ranged_speed: f32,
    #[serde(default)]
    pub rap: f32,
    #[serde(default)]
    pub ranged_hit: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Weaponskill {
    #[serde(default)]
    pub axe: f32,
    #[serde(default)]
    pub bow: f32,
    #[serde(default)]
    pub crossbow: f32,
    #[serde(default)]
    pub dagger: f32,
    #[serde(default)]
    pub gun: f32,
    #[serde(default)]
    pub mace: f32,
    #[serde(default)]
    pub sword: f32,
    #[serde(default)]
    pub axe2h: f32,
    #[serde(default)]
    pub mace2h: f32,
    #[serde(default)]
    pub sword2h: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Defense {
    #[serde(default)]
    pub armor: f32,
    #[serde(default)]
    pub armor_bonus: f32,
    #[serde(default)]
    pub defense: f32,
    #[serde(default)]
    pub dodge: f32,
    #[serde(default)]
    pub parry: f32,
    #[serde(default)]
    pub block_chance: f32,
    #[serde(default)]
    pub block_base: f32,
    #[serde(default)]
    pub block_bonus: f32
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Resistances {
    #[serde(default)]
    pub arcane: f32,
    #[serde(default)]
    pub fire: f32,
    #[serde(default)]
    pub frost: f32,
    #[serde(default)]
    pub nature: f32,
    #[serde(default)]
    pub shadow: f32
}

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub attributes: Attributes,
    pub spell: Spell,
    pub melee: Melee,
    pub ranged: Ranged,
    pub weaponskill: Weaponskill,
    pub defense: Defense,
    pub resistances: Resistances,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attributes {
    pub strength: f32,
    pub agility: f32,
    pub stamina: f32,
    pub intellect: f32,
    pub spirit: f32,
    pub health: f32,
    pub mana: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spell {
    pub spell_damage: f32,
    pub healing: f32,
    pub spell_hit: f32,
    pub spell_crit: f32,
    pub spell_pen: f32,
    pub mp5: f32,
    pub arcane: f32,
    pub fire: f32,
    pub frost: f32,
    pub nature: f32,
    pub shadow: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Melee {
    pub ap: f32,
    pub feral_ap: f32,
    pub weapon_dps: f32,
    pub weapon_speed: f32,
    pub hit: f32,
    pub crit: f32,
    pub haste: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ranged {
    pub ranged_dps: f32,
    pub ranged_speed: f32,
    pub rap: f32,
    pub ranged_hit: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weaponskill {
    pub axe: f32,
    pub bow: f32,
    pub crossbow: f32,
    pub dagger: f32,
    pub gun: f32,
    pub mace: f32,
    pub sword: f32,
    pub axe2h: f32,
    pub mace2h: f32,
    pub sword2h: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Defense {
    pub armor: f32,
    pub armor_bonus: f32,
    pub defense: f32,
    pub dodge: f32,
    pub parry: f32,
    pub block_chance: f32,
    pub block_base: f32,
    pub block_bonus: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resistances {
    pub arcane: f32,
    pub fire: f32,
    pub frost: f32,
    pub nature: f32,
    pub shadow: f32
}

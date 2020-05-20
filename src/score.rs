use std::cmp::Ordering;

use crate::gear::Gear;
use crate::stats::Stats;
use crate::restriction::Restriction;

#[derive(Debug)]
pub struct Score {
    name: String,
    value: f32,
    upgrade: f32,
    replacement: f32,
}

impl Score {
    pub fn new(name: &str, value: f32) -> Self {
        Score {
            name: name.to_owned(),
            value: value,
            upgrade: 0.0,
            replacement: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn score(&self) -> i32 {
        let total = self.value + self.upgrade + self.replacement;
        total.round() as i32
    }

    pub fn set_upgrade(&mut self, upgrade: f32) {
        self.upgrade = upgrade;
    }

    pub fn set_replacement(&mut self, replacement: f32) {
        self.replacement = replacement;
    }
}

impl Eq for Score { }
impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.name == other.name && self.score() == other.score()
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        if self.score() < other.score() {
            return Ordering::Less;
        } else if self.score() > other.score() {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn calculate_score(class: &str, weights: &Stats, restrict: &Restriction,
                       gear: &Gear) -> Score {
    // check if the armor type can be worn
    if !gear.material.is_empty() && !restrict.armor.contains(&gear.material) {
        return Score::new(gear.name.as_str(), 0.0);
    }

    // check if its a weapon the class can hold
    if !gear.weapon.is_empty() && !restrict.weapons.contains(&gear.weapon) {
        return Score::new(gear.name.as_str(), 0.0);
    }

    // check if the gear can be worn by the class
    if !gear.restrict.is_empty() && gear.restrict != class {
        return Score::new(gear.name.as_str(), 0.0);
    }

    // calculate the dot product of weights and gear
    let score =
        gear.stats.attributes.strength * weights.attributes.strength +
        gear.stats.attributes.agility * weights.attributes.agility +
        gear.stats.attributes.stamina * weights.attributes.stamina +
        gear.stats.attributes.intellect * weights.attributes.intellect +
        gear.stats.attributes.spirit * weights.attributes.spirit +
        gear.stats.attributes.health * weights.attributes.health +
        gear.stats.attributes.mana * weights.attributes.mana +
        gear.stats.spell.spell_damage * weights.spell.spell_damage +
        gear.stats.spell.healing * weights.spell.healing +
        gear.stats.spell.spell_hit * weights.spell.spell_hit +
        gear.stats.spell.spell_crit * weights.spell.spell_crit +
        gear.stats.spell.spell_pen * weights.spell.spell_pen +
        gear.stats.spell.mp5 * weights.spell.mp5 +
        gear.stats.spell.arcane * weights.spell.arcane +
        gear.stats.spell.fire * weights.spell.fire +
        gear.stats.spell.frost * weights.spell.frost +
        gear.stats.spell.nature * weights.spell.nature +
        gear.stats.spell.shadow * weights.spell.shadow +
        gear.stats.melee.ap * weights.melee.ap +
        gear.stats.melee.feral_ap * weights.melee.feral_ap +
        gear.stats.melee.weapon_dps * weights.melee.weapon_dps +
        gear.stats.melee.weapon_speed * weights.melee.weapon_speed +
        gear.stats.melee.hit * weights.melee.hit +
        gear.stats.melee.crit * weights.melee.crit +
        gear.stats.melee.haste * weights.melee.haste +
        gear.stats.ranged.ranged_dps * weights.ranged.ranged_dps +
        gear.stats.ranged.ranged_speed * weights.ranged.ranged_speed +
        gear.stats.ranged.rap * weights.ranged.rap +
        gear.stats.ranged.ranged_hit * weights.ranged.ranged_hit +
        gear.stats.weaponskill.axe * weights.weaponskill.axe +
        gear.stats.weaponskill.bow * weights.weaponskill.bow +
        gear.stats.weaponskill.crossbow * weights.weaponskill.crossbow +
        gear.stats.weaponskill.dagger * weights.weaponskill.dagger +
        gear.stats.weaponskill.gun * weights.weaponskill.gun +
        gear.stats.weaponskill.mace * weights.weaponskill.mace +
        gear.stats.weaponskill.sword * weights.weaponskill.sword +
        gear.stats.weaponskill.axe2h * weights.weaponskill.axe2h +
        gear.stats.weaponskill.mace2h * weights.weaponskill.mace2h +
        gear.stats.weaponskill.sword2h * weights.weaponskill.sword2h +
        gear.stats.defense.armor * weights.defense.armor +
        gear.stats.defense.armor_bonus * weights.defense.armor_bonus +
        gear.stats.defense.defense * weights.defense.defense +
        gear.stats.defense.block_chance * weights.defense.block_chance +
        gear.stats.defense.block_base * weights.defense.block_base +
        gear.stats.defense.block_bonus * weights.defense.block_bonus +
        gear.stats.resistances.arcane * weights.resistances.arcane +
        gear.stats.resistances.fire * weights.resistances.fire +
        gear.stats.resistances.frost * weights.resistances.frost +
        gear.stats.resistances.nature * weights.resistances.nature +
        gear.stats.resistances.shadow * weights.resistances.shadow;
    Score::new(&gear.name, score)
}

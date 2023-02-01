// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let test = vec![0,0];
        if self.health <= 0 {
            if self.level >= 10 {
                Some(Player{health: 100, level: self.level, mana: Some(100)})
            } else {
                Some(Player{health: 100, level: self.level, mana: None})
            }
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_some() {
            if mana_cost > self.mana.unwrap() {
                0
            } else {
                self.mana = Some(self.mana.unwrap() - mana_cost);
                mana_cost * 2
            }
        } else {
            if self.health > mana_cost {
                self.health = self.health - mana_cost;
            } else {
                self.health = 0;
            }
            0
        }
    }
}

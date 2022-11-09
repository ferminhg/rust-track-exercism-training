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
        if self.health > 0 { return None; }
        let mana = match self.mana {
            None => None,
            _ => Some(100)
        };

        Some( Player { health: 100, mana, level: self.level})
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            self.health = match mana_cost > self.health {
                true => 0,
                false => self.health - mana_cost
            };
            return 0;
        }

        let _mana_before_spell = self.mana.expect("Mana shouldn't be None ");
        if mana_cost > _mana_before_spell { return 0; }

        self.mana = Some(_mana_before_spell - mana_cost);
        mana_cost * 2
    }
}

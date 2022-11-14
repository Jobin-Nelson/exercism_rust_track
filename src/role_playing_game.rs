#[derive(Debug)]
pub struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }
        let health = 100;
        let level = self.level;
        let mana = if level >= 10 { Some(100) } else { None };

        Some(Player {
            health,
            mana,
            level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(num) => {
                if num >= mana_cost {
                    self.mana = Some(num - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health = self.health - std::cmp::min(self.health, mana_cost);
                0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a_wizard_yet_test() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: None,
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
        assert_eq!(not_a_wizard_yet.health, 74);
        assert_eq!(not_a_wizard_yet.mana, None);

    }

    #[test]
    fn low_mana_wizard_test() {
        let mut low_mana_wizard = Player {
            health: 93,
            mana: Some(3),
            level: 12,
        };
        assert_eq!(low_mana_wizard.cast_spell(10), 0);
        assert_eq!(low_mana_wizard.health, 93);
        assert_eq!(low_mana_wizard.mana, Some(3));
    }

    #[test]
    fn wizard_test() {
        let mut wizard = Player {
            health: 123,
            mana: Some(30),
            level: 18,
        };
        assert_eq!(wizard.cast_spell(10), 20);
        assert_eq!(wizard.health, 123);
        assert_eq!(wizard.mana, Some(20));
    }
}

use crate::{
    GameError,
    card::{Card, CardKind},
};

#[derive(Debug)]
pub struct Weapon {
    pub strength: u8,
    pub last_slain_monster_strength: u8,
}

#[derive(Debug)]
pub struct Player {
    pub health: u8,
    pub weapon: Weapon,
    pub last_healed_turn: u8,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            health: 20,
            weapon: Weapon {
                strength: 0,
                last_slain_monster_strength: 0,
            },
            last_healed_turn: 0,
        };
    }

    pub fn fight(&mut self, card: &Card) -> Result<(), GameError> {
        if !matches!(card.kind, CardKind::Monster) {
            return Err(GameError::NotAMonster);
        }

        let damage = card.strength.min(self.health);
        self.health -= damage;

        Ok(())
    }

    pub fn kill(&mut self, card: &Card) -> Result<(), GameError> {
        if !matches!(card.kind, CardKind::Monster) {
            return Err(GameError::NotAMonster);
        }

        if self.weapon.strength == 0 {
            return Err(GameError::NoWeaponEquipped);
        }

        if self.weapon.last_slain_monster_strength != 0
            && card.strength >= self.weapon.last_slain_monster_strength
        {
            return Err(GameError::MonsterTooStrongForWeapon);
        }

        if card.strength > self.weapon.strength {
            self.health -= (card.strength - self.weapon.strength).min(self.health);
        }

        self.weapon.last_slain_monster_strength = card.strength;

        Ok(())
    }

    pub fn equip_weapon(&mut self, card: &Card) -> Result<(), GameError> {
        if !matches!(card.kind, CardKind::Weapon) {
            return Err(GameError::NotAWeapon);
        }

        self.weapon = Weapon {
            strength: card.strength,
            last_slain_monster_strength: 0,
        };

        Ok(())
    }

    pub fn heal(&mut self, card: &Card, turn: u8) -> Result<(), GameError> {
        if !matches!(card.kind, CardKind::Potion) {
            return Err(GameError::NotAPotion);
        }

        if self.last_healed_turn != turn {
            self.health = (self.health + card.strength).min(20);
            self.last_healed_turn = turn;
        }

        Ok(())
    }
}

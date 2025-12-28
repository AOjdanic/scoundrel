use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, CardKind, Rank, Suit};

pub mod card;

#[derive(Debug)]
pub struct Weapon {
    pub strength: u8,
    pub last_slain_monster_strength: u8,
}

#[derive(Debug)]
pub struct Deck {
    pub turn: u8,
    pub health: u8,
    pub turn_healed: u8,
    pub turn_skipped: u8,
    pub weapon: Weapon,
    pub room: Vec<Card>,
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub enum GameError {
    NotAWeapon,
    NotAPotion,
    NotAMonster,
    IndexOutOfBounds,
    NoWeaponEquipped,
    MonsterTooStrongForWeapon,
}

impl<'a> Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for suit in [Suit::Spades, Suit::Clubs] {
            for rank in [
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
                Rank::Num(2),
                Rank::Num(3),
                Rank::Num(4),
                Rank::Num(5),
                Rank::Num(6),
                Rank::Num(7),
                Rank::Num(8),
                Rank::Num(9),
                Rank::Num(10),
            ] {
                cards.push(Card::new(suit, rank));
            }
        }

        for suit in [Suit::Diamonds, Suit::Hearts] {
            for rank in [
                Rank::Num(2),
                Rank::Num(3),
                Rank::Num(4),
                Rank::Num(5),
                Rank::Num(6),
                Rank::Num(7),
                Rank::Num(8),
                Rank::Num(9),
                Rank::Num(10),
            ] {
                cards.push(Card::new(suit, rank));
            }
        }

        let mut rng = thread_rng();

        cards.shuffle(&mut rng);

        return Self {
            cards,
            room: Vec::new(),
            turn: 0,
            turn_skipped: 0,
            health: 20,
            weapon: Weapon {
                last_slain_monster_strength: 0,
                strength: 0,
            },
            turn_healed: 0,
        };
    }

    pub fn new_turn(&mut self) {
        self.turn += 1
    }

    pub fn deal(&mut self) {
        let iterations = 4 - self.room.len();

        for _ in 0..iterations {
            match self.cards.pop() {
                Some(card) => {
                    self.room.push(card);
                }
                None => continue,
            }
        }
    }

    pub fn can_skip(&self) -> bool {
        if self.room.len() < 4 {
            return false;
        }

        if self.turn != 1 && self.turn_skipped != 0 {
            if self.turn - self.turn_skipped == 1 {
                return false;
            }
        }

        return true;
    }

    pub fn skip(&mut self) {
        let mut remaining_cards_in_room = self.room.len();

        while remaining_cards_in_room != 0 {
            match self.room.pop() {
                Some(card) => self.cards.insert(0, card),
                None => continue,
            }

            remaining_cards_in_room -= 1
        }

        self.turn_skipped = self.turn;
    }

    pub fn fight(&mut self, index: usize) -> Result<(), GameError> {
        let card = self.room.get(index).ok_or(GameError::IndexOutOfBounds)?;

        if !matches!(card.kind, CardKind::Monster) {
            return Err(GameError::NotAMonster);
        }

        let damage = card.strength.min(self.health);
        self.health -= damage;

        self.room.remove(index);

        Ok(())
    }

    pub fn equip_weapon(&mut self, index: usize) -> Result<(), GameError> {
        let card = self.room.get(index).ok_or(GameError::IndexOutOfBounds)?;

        if !matches!(card.kind, CardKind::Weapon) {
            return Err(GameError::NotAWeapon);
        }

        self.weapon = Weapon {
            strength: card.strength,
            last_slain_monster_strength: 0,
        };

        self.room.remove(index);

        Ok(())
    }

    pub fn kill(&mut self, index: usize) -> Result<(), GameError> {
        let card = self.room.get(index).ok_or(GameError::IndexOutOfBounds)?;

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
        self.room.remove(index);

        Ok(())
    }

    pub fn heal(&mut self, index: usize) -> Result<(), GameError> {
        let card = self.room.get(index).ok_or(GameError::IndexOutOfBounds)?;

        if !matches!(card.kind, CardKind::Potion) {
            return Err(GameError::NotAPotion);
        }

        if self.turn_healed != self.turn {
            self.health = (self.health + card.strength).min(20);
            self.turn_healed = self.turn;
        }

        self.room.remove(index);

        Ok(())
    }

    pub fn calculate_score(&self) -> i16 {
        if self.cards.len() == 0 {
            return self.health.into();
        } else {
            let all_monsters_strength: u8 = self
                .cards
                .iter()
                .filter(|card| matches!(card.kind, CardKind::Monster))
                .map(|card| card.strength)
                .sum();

            return all_monsters_strength.into();
        }
    }
}

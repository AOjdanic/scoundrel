use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, process};

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

pub enum GameError {
    IndexOutOfBounds,
    NotAMonster,
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

    pub fn equip_weapon(&mut self) {
        let (card, index) = match self.find_card() {
            Some(v) => v,
            None => {
                println!("Can't find weapon at that position");
                return;
            }
        };

        if matches!(card.kind, CardKind::Weapon) == false {
            println!("Selected card is not a weapon");
            return;
        }

        self.weapon = Weapon {
            strength: card.strength,
            last_slain_monster_strength: 0,
        };

        self.discard_from_room(index);
    }

    pub fn kill(&mut self) {
        let (card, index) = match self.find_card() {
            Some(v) => v,
            None => {
                println!("No card at given position");
                return;
            }
        };

        if matches!(card.kind, CardKind::Monster) == false {
            println!("Pick a monster to kill");
            return;
        }

        if self.weapon.strength == 0 {
            println!("You must equip a weapon");
            return;
        }

        if self.weapon.last_slain_monster_strength != 0
            && card.strength >= self.weapon.last_slain_monster_strength
        {
            println!(
                "The monster is too strong, you can only fight monster that have strength less than {}",
                self.weapon.last_slain_monster_strength
            );
            return;
        }

        self.combat(card.strength);
        self.discard_from_room(index);
    }

    pub fn heal(&mut self) {
        let (card, index) = match self.find_card() {
            Some(v) => v,
            None => {
                println!("No card at given position");
                return;
            }
        };

        if matches!(card.kind, CardKind::Potion) == false {
            println!("You can only use potions to heal");
            return;
        }

        if self.turn_healed != self.turn {
            let new_health = self.health + card.strength;

            self.health = if new_health > 20 { 20 } else { new_health };
            self.turn_healed = self.turn;
        }

        self.discard_from_room(index);
    }

    fn find_card(&self) -> Option<(&Card, usize)> {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            println!("Failed reading the input");
            process::exit(1)
        });

        let position: u8 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => return None,
        };

        let index: usize = (position - 1).into();
        match self.room.get(index) {
            Some(c) => return Some((c, index)),
            None => return None,
        };
    }

    fn combat(&mut self, monster_strength: u8) {
        let diff;

        if monster_strength > self.weapon.strength {
            diff = monster_strength - self.weapon.strength;

            if diff > self.health {
                self.health = 0
            } else {
                self.health -= diff
            }
        }

        self.weapon.last_slain_monster_strength = monster_strength;
    }

    fn discard_from_room(&mut self, index: usize) {
        self.room.remove(index);
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

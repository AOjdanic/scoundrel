use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, process};

use crate::card::{Card, Rank, Suit};

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

pub struct Config {
    pub suits: Vec<Suit>,
    pub ranks: Vec<Rank>,
    pub excluded_suits: Vec<Suit>,
    pub excluded_ranks: Vec<Rank>,
}

impl<'a> Deck {
    pub fn new(config: &'a Config) -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for suit in &config.suits {
            for value in &config.ranks {
                cards.push(Card::new(suit, value));
            }
        }

        let mut cards: Vec<Card> = cards
            .into_iter()
            .filter(|card| {
                return !(config.excluded_suits.contains(&card.suit)
                    && config.excluded_values.contains(&card.value));
            })
            .collect();

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

    pub fn fight(&mut self) {
        let (card, index) = match self.find_card() {
            Some(v) => v,
            None => {
                println!("No monster at given position");
                return;
            }
        };

        if !is_monster(card) {
            println!("Pick a monster to fight");
            return;
        }

        if card.strength > self.health {
            self.health = 0
        } else {
            self.health -= card.strength;
        }

        self.discard_from_room(index);
    }

    pub fn equip_weapon(&mut self) {
        let (card, index) = match self.find_card() {
            Some(v) => v,
            None => {
                println!("Can't find weapon at that position");
                return;
            }
        };

        if !is_weapon(card) {
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

        if !is_monster(card) {
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

        if !is_potion(card) {
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
                .filter(|card| is_monster(card))
                .map(|card| card.strength)
                .sum();

            return all_monsters_strength.into();
        }
    }
}

fn is_weapon(card: &Card) -> bool {
    return card.suit == "♦";
}

fn is_monster(card: &Card) -> bool {
    return card.suit == "♠" || card.suit == "♣";
}

fn is_potion(card: &Card) -> bool {
    return card.suit == "♥";
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_room(deck: &Deck) {
    let mut dealt_cards: Vec<String> = Vec::new();

    deck.room.iter().for_each(|card| {
        let card_annotation = format!(" {}{} ", card.suit, card.value);
        dealt_cards.push(card_annotation);
    });

    let dealt_cards = dealt_cards.join(" ");

    clear_screen();
    println!("{}", dealt_cards);
    println!("health: {}", deck.health);
    println!(
        "weapon: {} | can fight below: {}",
        deck.weapon.strength, deck.weapon.last_slain_monster_strength
    );
    println!("cards in deck: {}", deck.cards.len());
    println!("turn: {}", deck.turn);
    println!("turn skipped: {}", deck.turn_skipped);
    println!("turn healed: {}", deck.turn_healed);
}

// const SUITS: [&'static str; 4] = ["♠", "♥", "♦", "♣"];
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_monster_test() {
        let monster_card_one = Card::new("♠", "11");
        let monster_card_two = Card::new("♣", "2");
        let potion_card = Card::new("♥", "2");
        let weapon_card = Card::new("♦", "5");

        assert_eq!(is_monster(&potion_card), false);
        assert_eq!(is_monster(&weapon_card), false);
        assert_eq!(is_monster(&monster_card_one), true);
        assert_eq!(is_monster(&monster_card_two), true);
    }

    #[test]
    fn is_weapon_test() {
        let monster_card_one = Card::new("♠", "11");
        let monster_card_two = Card::new("♣", "2");
        let potion_card = Card::new("♥", "2");
        let weapon_card = Card::new("♦", "5");

        assert_eq!(is_weapon(&weapon_card), true);
        assert_eq!(is_weapon(&potion_card), false);
        assert_eq!(is_weapon(&monster_card_one), false);
        assert_eq!(is_weapon(&monster_card_two), false);
    }

    #[test]
    fn is_potion_test() {
        let monster_card_one = Card::new("♠", "11");
        let monster_card_two = Card::new("♣", "2");
        let potion_card = Card::new("♥", "2");
        let weapon_card = Card::new("♦", "5");

        assert_eq!(is_potion(&potion_card), true);
        assert_eq!(is_potion(&weapon_card), false);
        assert_eq!(is_potion(&monster_card_one), false);
        assert_eq!(is_potion(&monster_card_two), false);
    }
}

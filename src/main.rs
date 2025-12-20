use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, process};

// player struct has all actions and health points
// deck has creation and shuffling
// then there is a game struct which contains info about the game itself, the current cards in the
// room, weapons equipped (?), last skipped turn and so on

#[derive(Debug)]
struct Card<'a> {
    suit: &'a str,
    value: &'a str,
    strength: i8,
}

impl<'a> Card<'a> {
    fn new(suit: &'a str, value: &'a str) -> Self {
        let strength = match value {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            v => v.parse().expect("should have been able to convert to i8"),
        };

        return Card {
            suit,
            value,
            strength,
        };
    }
}

#[derive(Debug)]
struct Weapon {
    strength: i8,
    last_slain_monster_strength: i8,
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
    room: Vec<Card<'a>>,
    turn: i8,
    last_skipped_turn: i8,
    health: i8,
    weapon: Weapon,
    heal_turn: i8,
}

impl<'a> Deck<'a> {
    const SUITS: [&'static str; 4] = ["♠", "♥", "♦", "♣"];

    const VALUES: [&'static str; 13] = [
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];

    const EXCLUDED_VALUES: [&'static str; 4] = ["A", "J", "Q", "K"];

    const EXCLUDED_SUITS: [&'static str; 2] = ["♥", "♦"];

    fn new() -> Deck<'a> {
        let mut cards: Vec<Card<'a>> = Vec::new();

        for suit in Self::SUITS {
            for value in Self::VALUES {
                cards.push(Card::new(suit, value));
            }
        }

        let mut cards: Vec<Card> = cards
            .into_iter()
            .filter(|card| {
                return !(Self::EXCLUDED_SUITS.contains(&card.suit)
                    && Self::EXCLUDED_VALUES.contains(&card.value));
            })
            .collect();

        let mut rng = thread_rng();

        cards.shuffle(&mut rng);

        return Self {
            cards,
            room: Vec::new(),
            turn: 0,
            last_skipped_turn: 0,
            health: 20,
            weapon: Weapon {
                last_slain_monster_strength: 0,
                strength: 0,
            },
            heal_turn: 0,
        };
    }

    fn tick(&mut self) {
        self.turn += 1
    }

    fn deal(&mut self) {
        let iterations = 4 - self.room.len();

        println!("turn: {}", self.turn);
        println!("skipped: {}", self.last_skipped_turn);

        for _ in 0..iterations {
            match self.cards.pop() {
                Some(card) => {
                    self.room.push(card);
                }
                None => continue,
            }
        }

        self.print_room();
    }

    fn print_room(&self) {
        let mut dealt_cards: Vec<String> = Vec::new();

        self.room.iter().for_each(|card| {
            let card_annotation = format!(" {}{} ", card.suit, card.value);
            dealt_cards.push(card_annotation);
        });

        let dealt_cards = dealt_cards.join(" ");

        println!("{}", dealt_cards)
    }

    fn can_skip(&self) -> bool {
        if self.turn != 1 && self.last_skipped_turn != 0 {
            if self.turn - self.last_skipped_turn == 1 {
                return false;
            }
        }

        return true;
    }

    fn skip(&mut self) {
        let mut iterations = self.room.len();

        while iterations != 0 {
            match self.room.pop() {
                Some(card) => self.cards.insert(0, card),
                None => continue,
            }

            iterations -= 1
        }

        self.last_skipped_turn = self.turn;
    }

    fn fight(&mut self) {
        let position = match get_position() {
            Some(v) => v,
            None => return,
        };

        let card = match self.get_card(position) {
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

        self.health -= card.strength;

        let index = position - 1;
        self.room.remove(index);
        self.print_room();
    }

    fn equip_weapon(&mut self) {
        let position = match get_position() {
            Some(v) => v,
            None => return,
        };

        let card = match self.get_card(position) {
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

        self.room.remove(position - 1);
        self.print_room();
    }

    fn get_card(&self, position: usize) -> Option<&Card<'a>> {
        let card: &Card;

        let index = position - 1;
        match self.room.get(index) {
            Some(c) => {
                card = c;
            }
            None => return None,
        };

        return Some(card);
    }

    fn kill(&mut self) {
        let position = match get_position() {
            Some(v) => v,
            None => return,
        };

        let card = match self.get_card(position) {
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

        let weapon: &Weapon = self.get_weapon();

        if weapon.strength == 0 {
            println!("You must equip a weapon");
            return;
        }

        if weapon.last_slain_monster_strength != 0
            && card.strength > weapon.last_slain_monster_strength
        {
            println!(
                "The monster is too strong, you can only fight monster that have strength less than {}",
                weapon.last_slain_monster_strength
            );
            return;
        }

        self.update(card.strength);

        let index = position - 1;
        self.room.remove(index);
        self.print_room();
    }

    fn heal(&mut self) {
        let position = match get_position() {
            Some(v) => v,
            None => return,
        };

        let card = match self.get_card(position) {
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

        if self.heal_turn != self.turn {
            let new_health = self.health + card.strength;

            self.health = if new_health > 20 { 20 } else { new_health };
            self.heal_turn = self.turn;
        }

        let index = position - 1;
        self.room.remove(index);
        self.print_room();
    }

    fn update(&mut self, monster_strength: i8) {
        let difference = if (self.weapon.strength - monster_strength) < 0 {
            self.weapon.strength - monster_strength
        } else {
            0
        };

        self.health += difference;
        self.weapon.last_slain_monster_strength = monster_strength;
    }

    fn get_weapon(&self) -> &Weapon {
        return &self.weapon;
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

fn get_position() -> Option<usize> {
    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read input");

    match position.trim().parse() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn main() {
    let mut deck = Deck::new();

    'outer: loop {
        deck.tick();
        deck.deal();

        'inner: loop {
            let mut action = String::new();
            io::stdin().read_line(&mut action).unwrap();

            match action.trim() {
                "q" => break 'outer,
                "s" => {
                    if !deck.can_skip() {
                        println!("Can't skip two rooms in a row");
                        continue;
                    }

                    deck.skip();
                    break 'inner;
                }
                "e" => {
                    println!("Submit the position of the weapon you want to equip");

                    deck.equip_weapon();
                }
                "a" => {
                    println!("Submit the position of the monster you want to kill");

                    deck.kill();
                }
                "h" => {
                    println!("Submit the position of the potion you want to use");

                    deck.heal();
                }
                "f" => {
                    println!("Submit the position of the monster you want to fight bare handed");

                    deck.fight();
                }
                _ => {
                    println!("invalid action");
                }
            }

            if deck.health <= 0 {
                println!("You lose");
                process::exit(0);
            }

            if deck.room.len() == 1 {
                break 'inner;
            }
        }
    }
}

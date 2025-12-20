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
    strength: u8,
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
    strength: u8,
    last_slain_monster_strength: u8,
}

#[derive(Debug)]
struct Deck<'a> {
    turn: u8,
    health: u8,
    turn_healed: u8,
    turn_skipped: u8,
    weapon: Weapon,
    room: Vec<Card<'a>>,
    cards: Vec<Card<'a>>,
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
            turn_skipped: 0,
            health: 20,
            weapon: Weapon {
                last_slain_monster_strength: 0,
                strength: 0,
            },
            turn_healed: 0,
        };
    }

    fn new_turn(&mut self) {
        self.turn += 1
    }

    fn deal(&mut self) {
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

    fn can_skip(&self) -> bool {
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

    fn skip(&mut self) {
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

    fn fight(&mut self) {
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

    fn equip_weapon(&mut self) {
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

    fn kill(&mut self) {
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

    fn heal(&mut self) {
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

    fn find_card(&self) -> Option<(&Card<'a>, usize)> {
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

    fn calculate_score(&self) -> i16 {
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

fn print_room(deck: &Deck) {
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

fn main() {
    let mut deck = Deck::new();

    'outer: loop {
        deck.new_turn();
        deck.deal();

        'inner: loop {
            print_room(&deck);
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Should have been able to read input");

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
                "f" => {
                    println!("Submit the position of the monster you want to fight bare handed");

                    deck.fight();
                }
                "h" => {
                    println!("Submit the position of the potion you want to use");

                    deck.heal();
                }
                _ => {
                    println!("invalid action");
                }
            }

            if deck.health <= 0 {
                println!("You lose");
                println!("Score: -{}", deck.calculate_score());
                process::exit(0);
            }

            if deck.cards.len() == 0 {
                println!("You win!");
                println!("Score: {}", deck.calculate_score());
                process::exit(0);
            }

            if deck.room.len() == 1 {
                break 'inner;
            }
        }
    }
}

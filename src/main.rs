use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

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
            v => v.parse().expect("should have been able to convert a u8"),
        };

        return Card {
            suit,
            value,
            strength,
        };
    }
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
    room: Vec<Card<'a>>,
    turn: u8,
    last_skipped_turn: u8,
    life: u8,
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
            life: 20,
        };
    }

    fn tick(&mut self) {
        self.turn += 1
    }

    fn is_monster(&self, card: &Card) -> bool {
        return card.suit == "♠" || card.suit == "♣";
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

    fn fight(&mut self, position: usize) {
        let card: &Card;

        let index = position - 1;
        match self.room.get(index) {
            Some(c) => card = c,
            None => {
                println!("No monster at given position");
                return;
            }
        };

        if self.is_monster(card) == false {
            println!("Pick a monster to fight")
        }

        self.life -= card.strength;

        println!("{}", self.life);

        self.room.remove(index);
        self.print_room();
    }

    fn kill(&self) {
        unimplemented!("fight is unimplemented")
    }
}

fn main() {
    let mut deck = Deck::new();

    'outer: loop {
        println!("life points: {}", deck.life);
        deck.tick();
        deck.deal();

        'inner: loop {
            let mut action = String::new();
            io::stdin().read_line(&mut action).unwrap();

            match action.trim() {
                "q" => break 'outer,
                "s" => {
                    if deck.can_skip() {
                        println!("skipped");
                        deck.skip();
                        break 'inner;
                    } else {
                        println!("Can't skip two rooms in a row");
                        continue;
                    }
                }
                "a" => {
                    println!("Select the monster you want to kill");
                    deck.kill();
                    continue;
                }
                "b" => {
                    println!("Submit position of the monster you want to fight bare handed");
                    let mut position = String::new();
                    io::stdin().read_line(&mut position).unwrap();

                    let position: usize = match position.trim().parse() {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    deck.fight(position);
                    continue;
                }
                _ => {
                    println!("what");
                    continue;
                }
            }
        }
    }
}

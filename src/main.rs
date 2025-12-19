use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(Debug)]
struct Card<'a> {
    suit: &'a str,
    value: &'a str,
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
    room: Vec<Card<'a>>,
    turn: u8,
    skip: u8,
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
                cards.push(Card { suit, value });
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

        return Deck {
            cards,
            room: Vec::new(),
            turn: 0,
            skip: 0,
        };
    }

    fn deal(&mut self) {
        let iterations = 4 - self.room.len();

        self.turn += 1;
        println!("turn: {}", self.turn);
        println!("skipped: {}", self.skip);

        for _ in 0..iterations {
            match self.cards.pop() {
                Some(card) => {
                    self.room.push(card);
                }
                None => continue,
            }
        }

        let mut dealt_cards: Vec<String> = Vec::new();

        self.room.iter().for_each(|card| {
            let card_annotation = format!(" {}{} ", card.suit, card.value);
            dealt_cards.push(card_annotation);
        });

        let dealt_cards = dealt_cards.join(" ");

        println!("{}", dealt_cards)
    }

    fn can_skip(&self) -> bool {
        if self.turn != 1 && self.skip != 0 {
            if self.turn - self.skip == 1 {
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

        self.skip = self.turn;
    }
}

fn main() {
    // let mut life_points = 20;

    let mut deck = Deck::new();

    'outer: loop {
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
                _ => {
                    println!("what");
                    continue;
                }
            }
        }
    }
}

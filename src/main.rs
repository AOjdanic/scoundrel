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
    cards_in_room: Vec<Card<'a>>,
    turn: u8,
    skipped_at_turn: u8,
}

impl<'a> Deck<'a> {
    const SUITS: [&'static str; 4] = ["Clubs", "Hearts", "Spades", "Diamonds"];

    const VALUES: [&'static str; 13] = [
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];

    const EXCLUDED_VALUES: [&'static str; 4] = ["A", "J", "Q", "K"];

    const EXCLUDED_SUITS: [&'static str; 2] = ["Hearts", "Diamonds"];

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
            cards_in_room: Vec::new(),
            turn: 0,
            skipped_at_turn: 0,
        };
    }

    fn deal(&mut self) {
        if self.turn != 1 && self.skipped_at_turn != 0 {
            if self.turn - self.skipped_at_turn == 1 {
                println!("Can't skip two rooms in a row");
                return;
            }
        }

        let iterations = 4 - self.cards_in_room.len();

        self.turn += 1;
        println!("turn: {}", self.turn);
        println!("skipped: {}", self.skipped_at_turn);

        for _ in 0..iterations {
            match self.cards.pop() {
                Some(card) => {
                    self.cards_in_room.push(card);
                }
                None => continue,
            }
        }

        let mut dealt_cards = String::new();

        self.cards_in_room.iter().for_each(|card| {
            let card_annotation = format!("{}-{}", card.suit, card.value);
            dealt_cards.push_str(&card_annotation);
        });

        println!("{}", dealt_cards)
    }

    fn skip(&mut self) {
        let mut iterations = self.cards_in_room.len();

        while iterations != 0 {
            match self.cards_in_room.pop() {
                Some(card) => self.cards.insert(0, card),
                None => continue,
            }

            iterations -= 1
        }

        self.skipped_at_turn = self.turn;
    }
}

fn main() {
    // let mut life_points = 20;

    let mut deck = Deck::new();
    let mut action = String::new();

    loop {
        deck.deal();
        io::stdin().read_line(&mut action).unwrap();

        match action.trim() {
            "quit" => break,
            "skip" => {
                deck.skip();
                continue;
            }
            _ => (),
        }
    }
}

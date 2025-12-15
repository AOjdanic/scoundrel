use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Card<'a> {
    suit: &'a str,
    value: &'a str,
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
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

        return Deck { cards };
    }
}

fn main() {
    // let mut life_points = 20;

    let deck = Deck::new();

    dbg!(&deck);

    let length = deck.cards.len();

    println!("{length}")
}

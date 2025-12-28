use rand::{seq::SliceRandom, thread_rng};

use crate::card::{Card, CardKind, Rank, Suit};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
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

        return Self { cards };
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn cards_mut(&mut self) -> &mut Vec<Card> {
        return &mut self.cards;
    }

    pub fn put_on_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    pub fn is_empty(&self) -> bool {
        return self.cards.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.cards.len();
    }

    pub fn remaining_monster_strength(&self) -> u8 {
        return self
            .cards
            .iter()
            .filter(|card| matches!(card.kind, CardKind::Monster))
            .map(|card| card.strength)
            .sum();
    }
}

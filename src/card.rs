#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy)]
pub enum CardKind {
    Monster,
    Weapon,
    Potion,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub strength: u8,
    pub kind: CardKind,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        let strength = match rank {
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
            Rank::Num(v) => v,
        };

        let kind = match suit {
            Suit::Spades | Suit::Clubs => CardKind::Monster,
            Suit::Hearts => CardKind::Potion,
            Suit::Diamonds => CardKind::Weapon,
        };

        return Self {
            kind,
            suit,
            rank,
            strength,
        };
    }
}

use crate::{GameError, card::Card};

pub struct Room {
    cards: Vec<Card>,
    max_size: usize,
}

impl Room {
    pub fn new() -> Self {
        return Self {
            cards: Vec::new(),
            max_size: 4,
        };
    }

    pub fn is_full(&self) -> bool {
        return self.cards.len() == self.max_size;
    }

    pub fn len(&self) -> usize {
        return self.cards.len();
    }

    pub fn get(&self, index: usize) -> Result<&Card, GameError> {
        self.cards.get(index).ok_or(GameError::IndexOutOfBounds)
    }

    pub fn remove(&mut self, index: usize) -> Result<Card, GameError> {
        if index >= self.cards.len() {
            return Err(GameError::IndexOutOfBounds);
        }

        Ok(self.cards.remove(index))
    }

    pub fn add(&mut self, card: Card) -> Result<(), GameError> {
        if self.is_full() {
            return Err(GameError::RoomFull);
        }

        self.cards.push(card);
        Ok(())
    }

    pub fn clear_into(&mut self, deck: &mut Vec<Card>) {
        deck.splice(0..0, self.cards.drain(..));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Card> {
        return self.cards.iter();
    }
}

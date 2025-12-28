use crate::{deck::Deck, error::GameError, player::Player, room::Room};

pub enum Action {
    Quit,
    Skip,
    Kill { index: usize },
    Heal { index: usize },
    Fight { index: usize },
    Equip { index: usize },
}

pub enum GameEvent {
    TurnEnded,
    RoomSkipped,
    ActionApplied,
}

pub enum GameOutcome {
    Win { score: i16 },
    Lose { score: i16 },
}

pub struct Game {
    deck: Deck,
    room: Room,
    player: Player,

    turn: u8,
    last_skipped_turn: u8,
}

impl Game {
    pub fn new() -> Self {
        let deck = Deck::new();
        let room = Room::new();
        let player = Player::new();

        return Self {
            deck,
            room,
            player,

            turn: 0,
            last_skipped_turn: 0,
        };
    }

    pub fn start_turn(&mut self) {
        self.turn += 1;
        self.fill_room();
    }

    pub fn apply(&mut self, action: Action) -> Result<GameEvent, GameError> {
        match action {
            Action::Quit => {
                self.player.health = 0;
                Ok(GameEvent::TurnEnded)
            }

            Action::Skip => {
                if !self.can_skip() {
                    return Err(GameError::CannotSkip);
                }

                self.room.clear_into(self.deck.cards_mut());
                self.last_skipped_turn = self.turn;

                Ok(GameEvent::TurnEnded)
            }

            Action::Fight { index } => {
                let card = self.room.get(index)?;
                self.player.fight(card)?;
                self.room.remove(index)?;

                if self.room.len() == 1 {
                    Ok(GameEvent::TurnEnded)
                } else {
                    Ok(GameEvent::ActionApplied)
                }
            }

            Action::Kill { index } => {
                let card = self.room.get(index)?;
                self.player.kill(card)?;
                self.room.remove(index)?;

                if self.room.len() == 1 {
                    Ok(GameEvent::TurnEnded)
                } else {
                    Ok(GameEvent::ActionApplied)
                }
            }

            Action::Heal { index } => {
                let card = self.room.get(index)?;
                self.player.heal(card, self.turn)?;
                self.room.remove(index)?;

                if self.room.len() == 1 {
                    Ok(GameEvent::TurnEnded)
                } else {
                    Ok(GameEvent::ActionApplied)
                }
            }

            Action::Equip { index } => {
                let card = self.room.get(index)?;
                self.player.equip_weapon(card)?;
                self.room.remove(index)?;

                if self.room.len() == 1 {
                    Ok(GameEvent::TurnEnded)
                } else {
                    Ok(GameEvent::ActionApplied)
                }
            }
        }
    }

    pub fn room(&self) -> &Room {
        return &self.room;
    }

    pub fn player(&self) -> &Player {
        return &self.player;
    }

    pub fn turn(&self) -> u8 {
        return self.turn;
    }

    pub fn is_over(&self) -> bool {
        return self.deck.is_empty() || self.player.health == 0;
    }

    pub fn outcome(&self) -> Option<GameOutcome> {
        if !self.is_over() {
            return None;
        }

        let score = self.calculate_score();

        if self.player.health == 0 {
            return Some(GameOutcome::Lose { score });
        } else {
            Some(GameOutcome::Win { score })
        }
    }

    fn can_skip(&self) -> bool {
        if !self.room.is_full() {
            return false;
        }

        if self.turn != 1 && self.turn - self.last_skipped_turn == 1 {
            return false;
        }

        return true;
    }

    fn fill_room(&mut self) {
        while !self.room.is_full() {
            match self.deck.draw() {
                Some(card) => {
                    let _ = self.room.add(card);
                }
                None => break,
            }
        }
    }

    fn calculate_score(&self) -> i16 {
        if self.deck.is_empty() {
            self.player.health as i16
        } else {
            self.deck.remaining_monster_strength() as i16
        }
    }
}

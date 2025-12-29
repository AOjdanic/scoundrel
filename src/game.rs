use crate::{card::Card, deck::Deck, error::GameError, player::Player, room::Room, ui::Action};

pub enum GameEvent {
    QuitGame,
    TurnEnded,
    ActionApplied,
    RulesPrinted,
}

pub enum GameOutcome {
    Win { score: i16 },
    Lose { score: i16 },
}

pub struct GameInfo {
    pub health: u8,
    pub remaining_cards: usize,
    pub weapon_strength: u8,
    pub last_slain: u8,
    pub turn: u8,
    pub last_skipped: u8,
    pub room_cards: Vec<Card>,
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
            Action::Quit => Ok(GameEvent::QuitGame),

            Action::Skip => {
                match self.can_skip() {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                self.room.clear_into(self.deck.cards_mut());
                self.last_skipped_turn = self.turn;

                Ok(GameEvent::TurnEnded)
            }

            Action::PrintRules => Ok(GameEvent::RulesPrinted),

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

    pub fn game_info(&self) -> GameInfo {
        return GameInfo {
            room_cards: self.room.current_room().to_vec(),
            turn: self.turn,
            health: self.player.health,
            last_slain: self.player.weapon.last_slain_monster_strength,
            remaining_cards: self.deck.len(),
            last_skipped: self.last_skipped_turn,
            weapon_strength: self.player.weapon.strength,
        };
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

    fn can_skip(&self) -> Result<(), GameError> {
        if !self.room.is_full() {
            return Err(GameError::CannotSkip);
        }

        if self.turn != 1 && self.turn - self.last_skipped_turn == 1 {
            return Err(GameError::CannotSkipTwoInRow);
        }

        Ok(())
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

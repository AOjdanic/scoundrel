use std::{fmt, io};

use crate::{
    error::{GameError, UiError},
    game::{Game, GameOutcome},
};

pub enum Action {
    Quit,
    Skip,
    Kill { index: usize },
    Heal { index: usize },
    Fight { index: usize },
    Equip { index: usize },
}

pub struct Reader;

impl Reader {
    pub fn read_input() -> Result<String, UiError> {
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .map_err(|_| UiError::InputReadFailed)?;

        return Ok(action.trim().to_string());
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_action(input: &str) -> Result<Action, UiError> {
        let lower = input.to_lowercase();
        let mut iter = lower.trim().split_whitespace();

        let command = iter.next().ok_or(UiError::EmptyInput)?;

        match command {
            "q" => Ok(Action::Quit),

            "s" => Ok(Action::Skip),

            "f" => Ok(Action::Fight {
                index: Self::parse_index(iter.next())?,
            }),

            "a" => Ok(Action::Kill {
                index: Self::parse_index(iter.next())?,
            }),

            "e" => Ok(Action::Equip {
                index: Self::parse_index(iter.next())?,
            }),

            "h" => Ok(Action::Heal {
                index: Self::parse_index(iter.next())?,
            }),

            _ => Err(UiError::UnknownCommand),
        }
    }

    fn parse_index(value: Option<&str>) -> Result<usize, UiError> {
        let raw = value.ok_or(UiError::MissingIndex)?;

        let parsed: usize = raw.parse().map_err(|_| UiError::InvalidIndex)?;

        if parsed == 0 {
            return Err(UiError::IndexStartsAtOne);
        }

        Ok(parsed - 1)
    }
}

pub struct Printer;

impl Printer {
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn print_room(game: &Game) {
        let room = game.room();
        let health = game.player().health;
        let weapon_strength = game.player().weapon.strength;
        let last_slain = game.player().weapon.last_slain_monster_strength;
        let turn = game.turn();
        let last_skipped = game.last_skipped();
        let remaining = game.cards_remaining();
        let mut index = 0;
        room.iter().for_each(|card| {
            index += 1;
            print!("{}) {:?}{:?}   ", index, card.suit, card.rank)
        });
        print!("\n");
        println!("health: {health}");
        println!("cards in deck: {remaining}");
        println!("weapon: {weapon_strength}, last slain: {last_slain}");
        println!("turn: {turn}, turn skipped: {last_skipped}");
    }

    pub fn print_outcome(outcome: GameOutcome) {
        match outcome {
            GameOutcome::Win { score } => {
                println!("You win!");
                println!("Score: {}", score);
            }
            GameOutcome::Lose { score } => {
                println!("You lose!");
                println!("Score: -{}", score);
            }
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            GameError::RoomFull => "The room is already full.",
            GameError::NotAWeapon => "Selected card is not a weapon.",
            GameError::NotAPotion => "Selected card is not a potion.",
            GameError::CannotSkip => "You cannot skip a room you already started playing.",
            GameError::CannotSkipTwoInRow => "You cannot skip two rooms in a row.",
            GameError::NotAMonster => "Selected card is not a monster.",
            GameError::IndexOutOfBounds => "There is no card at the given position.",
            GameError::NoWeaponEquipped => "You must equip a weapon first.",
            GameError::MonsterTooStrongForWeapon => "This monster is too strong for your weapon.",
        };

        write!(f, "{msg}")
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            UiError::EmptyInput => "Please enter a command.",
            UiError::UnknownCommand => "Unknown command.",
            UiError::MissingIndex => "You must provide a card position.",
            UiError::InvalidIndex => "There is no card at the given position.",
            UiError::IndexStartsAtOne => "Card positions start at 1.",
            UiError::InputReadFailed => "Failed to read input.",
        };

        write!(f, "{msg}")
    }
}

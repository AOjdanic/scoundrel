use std::io;

use crate::{
    error::UiError,
    game::{Game, GameOutcome},
};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub enum Action {
    Quit,
    Skip,
    Kill { index: usize },
    Heal { index: usize },
    Fight { index: usize },
    Equip { index: usize },
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

pub fn read_input() -> Result<String, UiError> {
    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .map_err(|_| UiError::InputReadFailed)?;

    return Ok(action.trim().to_string());
}

fn parse_index(value: Option<&str>) -> Result<usize, UiError> {
    let raw = value.ok_or(UiError::MissingIndex)?;

    let parsed: usize = raw.parse().map_err(|_| UiError::InvalidIndex)?;

    if parsed == 0 {
        return Err(UiError::IndexStartsAtOne);
    }

    Ok(parsed - 1)
}

pub fn parse_action(input: &str) -> Result<Action, UiError> {
    let lower = input.to_lowercase();
    let mut iter = lower.trim().split_whitespace();

    let command = iter.next().ok_or(UiError::EmptyInput)?;

    match command {
        "q" => Ok(Action::Quit),

        "s" => Ok(Action::Skip),

        "f" => Ok(Action::Fight {
            index: parse_index(iter.next())?,
        }),

        "a" => Ok(Action::Kill {
            index: parse_index(iter.next())?,
        }),

        "e" => Ok(Action::Equip {
            index: parse_index(iter.next())?,
        }),

        "h" => Ok(Action::Heal {
            index: parse_index(iter.next())?,
        }),

        _ => Err(UiError::UnknownCommand),
    }
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

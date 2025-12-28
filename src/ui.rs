use std::{error::Error, io};

use scoundrel::Deck;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_room(deck: &Deck) {
    let mut dealt_cards: Vec<String> = Vec::new();

    deck.room.iter().for_each(|card| {
        let card_annotation = format!(" {:?}{:?} ", card.suit, card.rank);
        dealt_cards.push(card_annotation);
    });

    let dealt_cards = dealt_cards.join(" ");

    clear_screen();
    println!("{}", dealt_cards);
    println!("health: {}", deck.health);
    println!(
        "weapon: {} | can fight below: {}",
        deck.weapon.strength, deck.weapon.last_slain_monster_strength
    );
    println!("cards in deck: {}", deck.cards.len());
    println!("turn: {}", deck.turn);
    println!("turn skipped: {}", deck.turn_skipped);
    println!("turn healed: {}", deck.turn_healed);
}

pub fn read_input() -> Result<String, Box<dyn Error>> {
    let mut action = String::new();

    io::stdin().read_line(&mut action)?;

    return Ok(action.trim().to_string());
}

pub enum Action {
    Quit,
    Skip,
    Kill { index: usize },
    Heal { index: usize },
    Fight { index: usize },
    Equip { index: usize },
}

fn parse_index(value: Option<&str>) -> Result<usize, Box<dyn Error>> {
    let raw = value.ok_or("Missing index")?;

    let parsed: usize = raw.parse()?;

    if parsed == 0 {
        return Err("Index must not start at 0".into());
    }

    Ok(parsed - 1)
}

pub fn parse_action(input: &str) -> Result<Action, Box<dyn Error>> {
    let iter = input.to_lowercase();
    let mut iter = iter.trim().split_whitespace();

    let command = iter.next().ok_or("Empty Input")?;

    match command {
        "q" => Ok(Action::Quit),
        "s" => Ok(Action::Skip),
        "f" => {
            let index = parse_index(iter.next())?;

            Ok(Action::Fight { index: index })
        }

        "a" => {
            let index = parse_index(iter.next())?;

            Ok(Action::Kill { index: index })
        }

        "e" => {
            let index = parse_index(iter.next())?;

            Ok(Action::Equip { index: index })
        }

        "h" => {
            let index = parse_index(iter.next())?;

            Ok(Action::Heal { index: index })
        }

        _ => Err("Unknown command".into()),
    }
}

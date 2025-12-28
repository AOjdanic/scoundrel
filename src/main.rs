use scoundrel::{deck::Deck, player::Player, room::Room};
use std::{error::Error, process};

use crate::ui::{Action, parse_action, print_room, read_input};

pub mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut deck = Deck::new();
    let mut room = Room::new();
    let mut player = Player::new();

    'outer: loop {
        deck.new_turn();
        deck.deal();

        if deck.cards.len() == 0 {
            println!("You win!");
            println!("Score: {}", deck.calculate_score());
            process::exit(0);
        }

        'inner: loop {
            print_room(&deck);
            let input = read_input()?;
            let action = parse_action(&input)?;

            match action {
                Action::Quit => break 'outer Ok(()),

                Action::Skip => {
                    if !deck.can_skip() {
                        println!("Can't skip two rooms in a row");
                        continue;
                    }

                    deck.skip();
                    break 'inner;
                }

                Action::Equip { index } => {
                    let card = room.get(index)?;
                    player.equip_weapon(card)?;
                    room.remove(index)?;
                    // if let Err(e) = deck.equip_weapon(index) {
                    //     println!("Error: {:?}", e);
                    // }
                }

                Action::Kill { index } => {
                    let card = room.get(index)?;
                    player.kill(card)?;
                    room.remove(index)?;
                    // if let Err(e) = deck.kill(index) {
                    //     println!("Error: {:?}", e)
                    // }
                }

                Action::Fight { index } => {
                    let card = room.get(index)?;
                    player.fight(card)?;
                    room.remove(index)?;
                    // if let Err(e) = deck.fight(index) {
                    //     println!("Error: {:?}", e)
                    // }
                }

                Action::Heal { index } => {
                    let card = room.get(index)?;
                    player.heal(card)?;
                    room.remove(index)?;
                    // if let Err(e) = deck.heal(index) {
                    //     println!("Error: {:?}", e)
                    // }
                }
            }

            if deck.health <= 0 {
                println!("You lose");
                println!("Score: -{}", deck.calculate_score());
                process::exit(0);
            }

            if deck.room.len() == 1 {
                break 'inner;
            }
        }
    }
}

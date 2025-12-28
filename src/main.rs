use scoundrel::Deck;
use std::{error::Error, process};

use crate::ui::{Action, parse_action, print_room, read_input};

pub mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut deck = Deck::new();

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
                    println!("Submit the position of the weapon you want to equip");

                    if let Err(e) = deck.equip_weapon(index) {
                        println!("Error: {:?}", e);
                    }
                }

                Action::Kill { index } => {
                    println!("Submit the position of the monster you want to kill");

                    if let Err(e) = deck.kill(index) {
                        println!("Error: {:?}", e)
                    }
                }

                Action::Fight { index } => {
                    println!("Submit the position of the monster you want to fight bare handed");

                    if let Err(e) = deck.fight(index) {
                        println!("Error: {:?}", e)
                    }
                }

                Action::Heal { index } => {
                    println!("Submit the position of the potion you want to use");

                    if let Err(e) = deck.heal(index) {
                        println!("Error: {:?}", e)
                    }
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

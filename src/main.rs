// player struct has all actions and health points
// deck has creation and shuffling
// then there is a game struct which contains info about the game itself, the current cards in the
// room, weapons equipped (?), last skipped turn and so on

use scoundrel::{Deck, print_room};
use std::{io, process};

fn main() {
    let mut deck = Deck::new();

    'outer: loop {
        deck.new_turn();
        deck.deal();

        'inner: loop {
            print_room(&deck);
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Should have been able to read input");

            match action.trim() {
                "q" => break 'outer,
                "s" => {
                    if !deck.can_skip() {
                        println!("Can't skip two rooms in a row");
                        continue;
                    }

                    deck.skip();
                    break 'inner;
                }
                "e" => {
                    println!("Submit the position of the weapon you want to equip");

                    deck.equip_weapon();
                }
                "a" => {
                    println!("Submit the position of the monster you want to kill");

                    deck.kill();
                }
                "f" => {
                    println!("Submit the position of the monster you want to fight bare handed");

                    deck.fight();
                }
                "h" => {
                    println!("Submit the position of the potion you want to use");

                    deck.heal();
                }
                _ => {
                    println!("invalid action");
                }
            }

            if deck.health <= 0 {
                println!("You lose");
                println!("Score: -{}", deck.calculate_score());
                process::exit(0);
            }

            if deck.cards.len() == 0 {
                println!("You win!");
                println!("Score: {}", deck.calculate_score());
                process::exit(0);
            }

            if deck.room.len() == 1 {
                break 'inner;
            }
        }
    }
}

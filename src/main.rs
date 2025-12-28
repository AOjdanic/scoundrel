use scoundrel::{
    game::{Game, GameEvent},
    ui::{parse_action, print_outcome, print_room, read_input},
};

use std::{error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();

    'outer: loop {
        game.start_turn();

        'inner: loop {
            print_room(game.room());
            let input = read_input()?;
            let action = parse_action(&input)?;

            match game.apply(action) {
                Ok(GameEvent::TurnEnded) => break 'inner,
                Ok(GameEvent::ActionApplied) => continue,
                Ok(GameEvent::QuitGame) => process::exit(0),
                Err(e) => println!("{:?}", e),
            }

            if game.is_over() {
                break 'outer;
            }
        }
    }

    print_outcome(game.outcome().unwrap());

    Ok(())
}

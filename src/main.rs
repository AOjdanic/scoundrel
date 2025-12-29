use scoundrel::{
    game::{Game, GameEvent},
    ui::{clear_screen, parse_action, print_outcome, print_room, read_input},
};

fn main() {
    let mut game = Game::new();

    'game: loop {
        clear_screen();
        game.start_turn();

        'turn: loop {
            print_room(&game);
            let input = match read_input() {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Fatal input error: {}", e);
                    break 'game;
                }
            };

            let action = match parse_action(&input) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("{}", e);
                    continue 'turn;
                }
            };

            match game.apply(action) {
                Ok(GameEvent::TurnEnded) => break 'turn,
                Ok(GameEvent::ActionApplied) => {}
                Ok(GameEvent::QuitGame) => break 'game,
                Err(e) => {
                    eprintln!("{}", e);
                    continue 'turn;
                }
            }

            if game.is_over() {
                break 'game;
            }
        }
    }

    if let Some(outcome) = game.outcome() {
        print_outcome(outcome);
    }
}

use scoundrel::{
    game::{Game, GameEvent},
    ui::{Parser, Printer, Reader},
};

fn main() {
    let mut game = Game::new();

    'game: loop {
        Printer::clear_screen();
        game.start_turn();

        'turn: loop {
            Printer::print_room(game.game_info());
            let input = match Reader::read_input() {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Fatal input error: {}", e);
                    break 'game;
                }
            };

            let action = match Parser::parse_action(&input) {
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
        Printer::print_outcome(outcome);
    }
}

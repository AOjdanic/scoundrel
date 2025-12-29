use scoundrel::{
    error::AppError,
    game::{Game, GameEvent},
    ui::{Parser, Printer, Reader},
};

fn main() {
    let mut game = Game::new();
    let mut printer = Printer::new();

    'game: loop {
        Printer::clear_screen();
        game.start_turn();

        'turn: loop {
            Printer::clear_screen();
            Printer::print_room(game.game_info());
            printer.print_errors();

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
                    printer.add_error(AppError::Ui(e));
                    continue 'turn;
                }
            };

            match game.apply(action) {
                Ok(GameEvent::TurnEnded) => {
                    if game.is_over() {
                        break 'game;
                    }

                    break 'turn;
                }
                Ok(GameEvent::ActionApplied) => {}
                Ok(GameEvent::QuitGame) => break 'game,
                Err(e) => {
                    printer.add_error(AppError::Game(e));
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

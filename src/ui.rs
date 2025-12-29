use std::{fmt, io};

use crate::{
    card::{Card, Rank, Suit},
    error::{AppError, GameError, UiError},
    game::{GameInfo, GameOutcome},
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

pub struct Printer {
    errors: Vec<AppError>,
}

impl Printer {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, e: AppError) {
        self.errors.push(e);
    }

    pub fn print_errors(&mut self) {
        for error in self.errors.drain(0..self.errors.len()) {
            eprintln!("{}", &error);
        }
    }

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn print_room(game_info: GameInfo) {
        const TOTAL_WIDTH: usize = 78;
        const CARD_AREA_WIDTH: usize = 53;
        const CARD_WIDTH: usize = 9;
        const INNER_WIDTH: usize = CARD_WIDTH - 2;
        const CARD_GAP: &str = " ";
        const DECK_GAP: &str = "    ";

        let center = |s: &str, width: usize| {
            let len = s.chars().count();
            if len >= width {
                s.to_string()
            } else {
                let pad = width - len;
                format!("{}{}{}", " ".repeat(pad / 2), s, " ".repeat(pad - pad / 2))
            }
        };

        let room_cards: Vec<String> = game_info.room_cards.iter().map(|c| c.to_string()).collect();

        // ----- Card area (6 rows) -----
        let mut card_lines = vec![String::new(); 6];

        // Deck
        card_lines[0].push_str("+-------+");
        card_lines[1].push_str("|       |");
        card_lines[2].push_str(&format!(
            "|{}|",
            center(&game_info.remaining_cards.to_string(), INNER_WIDTH)
        ));
        card_lines[3].push_str("|       |");
        card_lines[4].push_str("+-------+");
        card_lines[5].push_str("         ");

        for line in &mut card_lines {
            line.push_str(DECK_GAP);
        }

        // Room cards
        for (idx, card) in room_cards.iter().enumerate() {
            card_lines[0].push_str("+-------+");
            card_lines[1].push_str("|       |");
            card_lines[2].push_str(&format!("|{}|", center(card, INNER_WIDTH)));
            card_lines[3].push_str("|       |");
            card_lines[4].push_str("+-------+");
            card_lines[5].push_str(&center(&(idx + 1).to_string(), CARD_WIDTH));

            for line in &mut card_lines {
                line.push_str(CARD_GAP);
            }
        }

        // ----- Stats: one per row, RIGHT-ALIGNED -----
        let stats = [
            format!("♡ {:>2}", game_info.health),
            format!("↺ {:>2}", game_info.turn),
            format!("⏭ {:>2}", game_info.last_skipped),
            format!("⚔ {:>2}", game_info.weapon_strength),
            format!("🥊{:>2}", game_info.last_slain),
            String::new(),
        ];

        // ===== RENDER =====
        println!("{:-<81}", "");
        for i in 0..card_lines.len() {
            if i == card_lines.len() - 2 {
                let left = format!("{:<CARD_AREA_WIDTH$}", card_lines[i]);
                let right = format!(
                    "{:>width$}",
                    stats[i],
                    width = TOTAL_WIDTH - CARD_AREA_WIDTH - 1
                );
                println!("|{}{} |", left, right);
            } else {
                let left = format!("{:<CARD_AREA_WIDTH$}", card_lines[i]);
                let right = format!(
                    "{:>width$}",
                    stats[i],
                    width = TOTAL_WIDTH - CARD_AREA_WIDTH
                );
                println!("|{}{} |", left, right);
            }
        }
        println!("{:-<81}", "");
        println!(
            "a = attack with weapon  f = fight barehanded  s = skip  e = equip  h = heal  r = rules"
        );
        println!();
        println!("example commands:");
        println!("s   = skip room");
        println!("e 2 = equip a weapon at position 2");
        println!("a 1 = attack monster at position 1");
        println!();
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

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            AppError::Ui(e) => format!("{}", e),
            AppError::Game(e) => format!("{}", e),
        };

        write!(f, "{msg}")
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
        };

        write!(f, "{msg}")
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            Rank::Num(v) => &v.to_string(),
        };

        write!(f, "{msg}")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("{}{}", self.suit, self.rank);
        write!(f, "{msg}")
    }
}

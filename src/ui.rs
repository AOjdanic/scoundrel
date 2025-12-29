use std::{fmt, io};

use crate::{
    card::{Card, Rank, Suit},
    error::{AppError, GameError, UiError},
    game::{GameInfo, GameOutcome},
};

pub enum Action {
    Quit,
    Skip,
    PrintRules,
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

            "r" => Ok(Action::PrintRules),

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
    pub fn print_rules() {
        let rules_lines = [
            "Scoundrel is a deck based dungeon crawler game.\n",
            "It is played with a standard deck of playing cards.",
            "The deck consists of 44 cards, with all Jokers, Red Face Cards and Red Aces removed.\n",
            "The deck is called the Dungeon.",
            "You as a player begin with 20 life points.\n",
            "Bules:\n",
            "The 26 Clubs and Spades in the deck are Monsters.",
            "Their damage is equal to their ordered value. (e.g. 10 is 10, Jack is 11, Queen is 12, King is 13, and Ace is 14)\n",
            "The 9 Diamonds in the deck are Weapons. Each weapon does as much damage as its value.",
            "All weapons in Scoundrel are binding, meaning if you pick one up, you must equip it, and discard your previous weapon.\n",
            "The 9 Hearts in the deck are Health Potions. You may only use one health potion each turn, even if you pull two.",
            "The second potion you use is simply discarded. You may not restore your life beyond your starting 20 health.\n",
            "The Game ends when either your life reaches zero or you make your way through the entire Dungeon.\n",
            "Scoring:\n",
            "If your life has reached zero, your score is the negative sum of all the remaining monsters in the Dungeon.",
            "If you have made your way through the entire dungeon, your score is equal to your remaining health points.",
            "If the deck runs out, and the last cards room is comprised of the remaining 4 or less cards, you don't have to clear that room. You win by default.",
            "Gameplay:\n",
            "On start of each turn, cards from the deck are drawn until there are 4 cards face up. These 4 cards represent the Room.",
            "You may avoid the Room if you wish. If you choose to do so, all four cards in the room will be placed at the bottom of the deck.",
            "You may avoid as many Rooms as you want, but you may not avoid two Rooms in a row.",
            "If you choose not to avoid the Room, you must face 3 of the four cards it contains, one at a time.\n",
            "If you chose a Weapon:",
            "You must equip it. If you had a previous Weapon equipped, it is discarded.\n",
            "If you chose a Health Potion:",
            "Add its number to your health, and then discard it. Your health may not exceed 20, and you may not use more than one Health Potion per turn.",
            "If you take two Health Potions on a single turn, the second is simply discarded, adding nothing to your health.\n",
            "If you chose a Monster:",
            "You may either fight it barehanded or with an equipped Weapon. Even if you have a Weapon equipped, you can still fight a monster barehanded.",
            "Combat:\n",
            "If you choose to fight the Monster barehanded, your health is diminished by the full value of that monster.",
            "If you choose to fight the Monster with your equipped Weapon, then your health is diminished by the difference in strength between the monster and the weapon (if weapon is weaker than the monster) or stays unchanged (if weapon is stronger than the monster)\n",
            "When you have just equipped a Weapon and haven't fought any monsters, then you can fight any monster with that Weapon.",
            "But, if you have fought a monster, next monster you decide to fight with that weapon has to be of strength less than that of the previous monster.\n",
            "For example, if your Weapon is a 5, you can fight any monster, even an Ace. If you fight and Ace you lose 9 health points (difference between Ace strength and weapon strength). Then, if you decide to fight another Ace, you can't do that with the current weapon. The current weapon can only fight monster weaker than Ace. If you fight a monster of strength 2, you take no damage, but with that weapon you can no longer fight any monster, as there is no monster weaker than 2.\n",
            "In that case, you will either have to equip a new weapon or fight barehanded.\n",
            "Once you have chosen 3 cards (such that only one remains), the turn is complete. The fourth card remains as part of the next Room.\n",
            "Legend:\n",
            " ♡ - health",
            " ⚔ - weapon strength",
            " 🥊 - weapon can fight below",
            " ⏭  - turn when last room skipped",
            " ↺ - turn number",
        ];
        println!();
        for line in rules_lines {
            println!("{}", line)
        }
        println!();
        println!("Submit any key to go back to game");
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

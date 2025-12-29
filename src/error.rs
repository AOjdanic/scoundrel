use std::fmt;

#[derive(Debug)]
pub enum GameError {
    RoomFull,
    NotAWeapon,
    NotAPotion,
    CannotSkip,
    CannotSkipTwoInRow,
    NotAMonster,
    IndexOutOfBounds,
    NoWeaponEquipped,
    MonsterTooStrongForWeapon,
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

impl std::error::Error for GameError {}

#[derive(Debug)]
pub enum UiError {
    EmptyInput,
    UnknownCommand,
    MissingIndex,
    InvalidIndex,
    IndexStartsAtOne,
    InputReadFailed,
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

impl std::error::Error for UiError {}

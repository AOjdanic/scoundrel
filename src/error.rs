use std::fmt;

#[derive(Debug)]
pub enum GameError {
    RoomFull,
    NotAWeapon,
    NotAPotion,
    CannotSkip,
    NotAMonster,
    IndexOutOfBounds,
    NoWeaponEquipped,
    MonsterTooStrongForWeapon,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            GameError::RoomFull => "The room is already full.",
            GameError::NotAWeapon => "That card is not a weapon.",
            GameError::NotAPotion => "That card is not a potion.",
            GameError::CannotSkip => "You cannot skip two rooms in a row.",
            GameError::NotAMonster => "That card is not a monster.",
            GameError::IndexOutOfBounds => "There is no card at that position.",
            GameError::NoWeaponEquipped => "You must equip a weapon first.",
            GameError::MonsterTooStrongForWeapon => "This monster is too strong for your weapon.",
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for GameError {}

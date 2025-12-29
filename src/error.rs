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

impl std::error::Error for UiError {}

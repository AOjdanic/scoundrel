#[derive(Debug)]
pub enum GameError {
    RoomFull,
    NotAWeapon,
    NotAPotion,
    NotAMonster,
    IndexOutOfBounds,
    NoWeaponEquipped,
    MonsterTooStrongForWeapon,
}

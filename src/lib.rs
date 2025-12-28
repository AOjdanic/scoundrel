pub mod card;
pub mod deck;
pub mod player;
pub mod room;

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

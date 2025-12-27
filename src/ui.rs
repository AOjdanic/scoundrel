use scoundrel::Deck;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_room(deck: &Deck) {
    let mut dealt_cards: Vec<String> = Vec::new();

    deck.room.iter().for_each(|card| {
        let card_annotation = format!(" {:?}{:?} ", card.suit, card.rank);
        dealt_cards.push(card_annotation);
    });

    let dealt_cards = dealt_cards.join(" ");

    clear_screen();
    println!("{}", dealt_cards);
    println!("health: {}", deck.health);
    println!(
        "weapon: {} | can fight below: {}",
        deck.weapon.strength, deck.weapon.last_slain_monster_strength
    );
    println!("cards in deck: {}", deck.cards.len());
    println!("turn: {}", deck.turn);
    println!("turn skipped: {}", deck.turn_skipped);
    println!("turn healed: {}", deck.turn_healed);
}

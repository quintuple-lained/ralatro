use uuid::Uuid;

mod card;
mod deck;

use card::{Card, CardEdition, CardEnhancement, CardSeal, Rank, Suit};
use deck::Deck;

fn main() {
    println!("=== Creating a Standard Deck ===");
    let mut deck = Deck::new_standard();
    println!("Deck has {} cards\n", deck.cards.len());

    println!("=== First 5 cards (before shuffle) ===");
    for card in deck.cards.iter().take(5) {
        println!("{:?} of {:?}", card.rank, card.suit);
    }
    println!();

    println!("=== Shuffling the deck ===");
    deck.shuffle();
    println!("Deck shuffled!\n");

    println!("=== First 5 cards (after shuffle) ===");
    for card in deck.cards.iter().take(5) {
        println!("{:?} of {:?}", card.rank, card.suit);
    }
    println!();

    println!("=== Taking top 5 cards ===");
    let hand = deck.take_top(5);
    println!("Drew {} cards:", hand.len());
    for card in &hand {
        println!("  {:?} of {:?} (UUID: {})", card.rank, card.suit, card.uuid);
    }
    println!("Deck now has {} cards\n", deck.cards.len());

    println!("=== Taking a random card ===");
    if let Some(random_card) = deck.take_random() {
        println!(
            "Random card: {:?} of {:?}",
            random_card.rank, random_card.suit
        );
        println!("Deck now has {} cards\n", deck.cards.len());
    }

    println!("=== Taking a specific card by UUID ===");
    let target_uuid = deck.cards[0].uuid; // Get UUID of first card
    let target_card_display = format!("{:?} of {:?}", deck.cards[0].rank, deck.cards[0].suit);

    if let Some(specific_card) = deck.take_specific(&target_uuid) {
        println!("Removed specific card: {}", target_card_display);
        println!("UUID was: {}", specific_card.uuid);
        println!("Deck now has {} cards\n", deck.cards.len());
    }

    println!("=== Adding a card back to the deck ===");
    let new_card = Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
        edition: CardEdition::Foil,
        enhancement: CardEnhancement::Gold,
        seal: CardSeal::Red,
        misc_bonus: 10,
        uuid: Uuid::now_v7(),
    };
    println!(
        "Adding: {:?} of {:?} (Foil, Gold, Red Seal)",
        new_card.rank, new_card.suit
    );
    deck.add_card(new_card);
    println!("Deck now has {} cards\n", deck.cards.len());

    println!("=== Last card in deck (the one we just added) ===");
    if let Some(last_card) = deck.cards.last() {
        println!(
            "  It is a {:?} Edition {:?} of {:?} {:?} Card with a {:?} Seal",
            last_card.edition,
            last_card.rank,
            last_card.suit,
            last_card.enhancement,
            last_card.seal
        );
        println!("  it is worth {:?} chips", last_card.chip_value())
    }

    println!("\n=== Final deck size: {} cards ===", deck.cards.len());
}

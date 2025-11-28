use rand::{Rng, rng, seq::SliceRandom};
use uuid::Uuid;

#[derive(Clone, Debug)]
enum CardEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
}

#[derive(Clone, Debug)]
enum CardEnhancement {
    None,
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

#[derive(Clone, Debug)]
enum CardSeal {
    None,
    Red,
    Blue,
    Gold,
    Purple,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
}

#[derive(Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
    edition: CardEdition,
    enhancement: CardEnhancement,
    seal: CardSeal,
    uuid: Uuid,
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new_standard() -> Self {
        Self {
            cards: create_standard_deck(),
        }
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn take_top(&mut self, amount: usize) -> Vec<Card> {
        self.cards
            .split_off(self.cards.len().saturating_sub(amount))
    }

    fn take_specific(&mut self, uuid: &Uuid) -> Option<Card> {
        self.cards
            .iter()
            .position(|c| &c.uuid == uuid)
            .map(|idx| self.cards.remove(idx))
    }

    fn take_random(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            let idx = rng().random_range(0..self.cards.len());
            Some(self.cards.remove(idx))
        }
    }
}

fn create_standard_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);

    for suit in Suit::all() {
        for rank in Rank::all() {
            deck.push(Card {
                rank,
                suit,
                edition: CardEdition::Base,
                enhancement: CardEnhancement::None,
                seal: CardSeal::None,
                uuid: Uuid::now_v7(),
            })
        }
    }
    deck
}

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
        )
    }

    println!("\n=== Final deck size: {} cards ===", deck.cards.len());
}

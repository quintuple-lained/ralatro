use crate::card::{Card, CardEdition, CardEnhancement, CardSeal, Rank, Suit};
use rand::seq::SliceRandom;
use rand::{Rng, rng};
use uuid::Uuid;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new_standard() -> Self {
        Self {
            cards: create_standard_deck(),
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn take_top(&mut self, amount: usize) -> Vec<Card> {
        self.cards
            .split_off(self.cards.len().saturating_sub(amount))
    }

    pub fn take_specific(&mut self, uuid: &Uuid) -> Option<Card> {
        self.cards
            .iter()
            .position(|c| &c.uuid == uuid)
            .map(|idx| self.cards.remove(idx))
    }

    pub fn take_random(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            let idx = rng().random_range(0..self.cards.len());
            Some(self.cards.remove(idx))
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    // Expose cards for iteration (but not mutation)
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

fn create_standard_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);

    for suit in Suit::all() {
        for rank in Rank::all() {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

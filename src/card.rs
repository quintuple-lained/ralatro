use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum CardEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
}

#[derive(Clone, Debug)]
pub enum CardEnhancement {
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
pub enum CardSeal {
    None,
    Red,
    Blue,
    Gold,
    Purple,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
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
    pub fn all() -> [Rank; 13] {
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

    pub fn chip_value(&self) -> u32 {
        match self {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub edition: CardEdition,
    pub enhancement: CardEnhancement,
    pub seal: CardSeal,
    pub misc_bonus: u128,
    pub uuid: Uuid,
}

impl Card {
    pub fn chip_value(&self) -> u128 {
        let base: u128 = self.rank.chip_value().into();

        let enhanced: u128 = match self.enhancement {
            CardEnhancement::Bonus => (base + 30).into(),
            CardEnhancement::Stone => 50,
            CardEnhancement::Steel => base,
            _ => base,
        };

        let edition_bonus: u128 = match self.edition {
            CardEdition::Foil => 50,
            CardEdition::Holographic => 10,
            CardEdition::Polychrome => 0,
            CardEdition::Base => 0,
        };

        enhanced + edition_bonus + self.misc_bonus
    }

    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            edition: CardEdition::Base,
            enhancement: CardEnhancement::None,
            seal: CardSeal::None,
            misc_bonus: 0,
            uuid: Uuid::now_v7(),
        }
    }
}

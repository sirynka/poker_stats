use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    HighCard(Card),
    Pair([Card; 2]),
    TwoPair([Card; 2], [Card; 2]),
    ThreeOfAKind([Card; 3]),
    Straight([Card; 5]),
    Flush([Card; 5]),
    FullHouse([Card; 3], [Card; 2]),
    FourOfAKind([Card; 4]),
    StraightFlush([Card; 5]),
    RoyalFlush([Card; 5]),
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

pub struct DeckFormatter<'a> {
    pub deck: &'a Deck,
    pub rows: &'a usize,
}

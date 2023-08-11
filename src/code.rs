use crate::data::{Card, Deck, DeckFormatter, Hand, Rank, Suit};
use itertools::Itertools;
use std::convert::TryInto;
use std::fmt::Display;
use std::vec;
use strum::IntoEnumIterator;

#[allow(dead_code)]
impl Deck {
    pub fn french() -> Self {
        let suits = Suit::iter();
        let ranks = Rank::iter();
        let cards = itertools::iproduct!(ranks, suits)
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>();

        Deck { cards }
    }

    pub fn russian() -> Self {
        let suits = Suit::iter();
        let ranks = Rank::iter().filter(|rank| {
            use Rank::*;
            match rank {
                Two | Three | Four | Five => false,
                _ => true,
            }
        });
        let cards = itertools::iproduct!(ranks, suits)
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>();

        Deck { cards }
    }

    pub fn shuffle(mut self) -> Self {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        self
    }

    pub fn sort(mut self) -> Self {
        self.cards.sort();
        self
    }

    pub fn deal(&mut self, n: usize) -> Self {
        let cards = self.cards.drain(0..n).collect::<Vec<_>>();
        Deck { cards }
    }

    pub fn rows<'a>(&'a self, rows: &'a usize) -> DeckFormatter<'a> {
        DeckFormatter { deck: self, rows }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.cards.extend(other.cards.into_iter());
        self
    }

    pub fn best_hand(table: &Deck, hand: Deck) -> Hand {
        let Some(hand) = Self::hand_iter(table, hand).next() else {
            unreachable!("One card should always be present");
        };

        hand
    }

    pub fn all_hands(table: &Deck, hand: Deck) -> Vec<Hand> {
        Self::hand_iter(table, hand).collect()
    }

    fn hand_iter(table: &Deck, hand: Deck) -> impl Iterator<Item = Hand> {
        if hand.cards.len() != 2 {
            panic!("Hand must have exactly 2 cards");
        }

        let table_has_3_to_5_cards =
            3 <= table.cards.len() && table.cards.len() <= 5;
        if !table_has_3_to_5_cards {
            panic!("Table must have between 3 and 5 cards");
        }

        let table = table.clone().merge(hand).sort();
        let hands = vec![
            Self::royal_flush,
            Self::straight_flush,
            Self::four_of_a_kind,
            Self::full_house,
            Self::flush,
            Self::straight,
            Self::three_of_a_kind,
            Self::two_pairs,
            Self::pair,
            Self::highest_card,
        ];

        let hands = hands
            .into_iter()
            .filter_map(move |hand| hand(&table));

        hands
    }

    fn same_rank<const N: usize>(cards: &Vec<Card>) -> Option<[Card; N]> {
        cards
            .windows(N)
            .filter(|cards| cards.iter().all(|card| card.rank == cards[0].rank))
            .last()
            .map(|cards| cards.to_owned().try_into().ok())
            .flatten()
    }

    fn same_suit<const N: usize>(cards: &Vec<Card>) -> Option<[Card; N]> {
        let cards: Vec<_> = cards
            .iter()
            .sorted_by_key(|card| card.suit)
            .cloned()
            .collect();

        let cards = cards
            .windows(N)
            .filter(|cards| cards.iter().all(|card| card.suit == cards[0].suit))
            .last()
            .map(|cards| cards.to_owned().try_into().ok())
            .flatten();

        cards
    }

    fn consecutive<const N: usize>(cards: &Vec<Card>) -> Option<[Card; N]> {
        let diffs = cards
            .windows(2)
            .enumerate()
            .map(|(i, cards)| (i, cards[1].rank as i8 - cards[0].rank as i8))
            .collect::<Vec<_>>();

        let (idx, _) = diffs
            .windows(N - 1)
            .filter(|diffs| diffs.iter().all(|(_, diff)| *diff == 1 || *diff == -12))
            .last()?
            .first()?
            .to_owned();

        let straight: [Card; N] = cards[idx..idx + N].to_owned().try_into().ok()?;

        return Some(straight);
    }

    fn highest_card(&self) -> Option<Hand> {
        self.cards.last().cloned().map(Hand::HighCard)
    }

    fn pair(&self) -> Option<Hand> {
        let pair: [Card; 2] = Self::same_rank(&self.cards)?;
        Some(Hand::Pair(pair))
    }

    fn two_pairs(&self) -> Option<Hand> {
        let mut cards = self.cards.clone();
        let first_pair: [Card; 2] = Self::same_rank(&cards)?;

        cards.retain(|card| card.rank != first_pair[0].rank);
        let second_pair: [Card; 2] = Self::same_rank(&cards)?;

        Some(Hand::TwoPair(first_pair, second_pair))
    }

    fn three_of_a_kind(&self) -> Option<Hand> {
        let three_of_a_kind: [Card; 3] = Self::same_rank(&self.cards)?;
        Some(Hand::ThreeOfAKind(three_of_a_kind))
    }

    fn straight(&self) -> Option<Hand> {
        if let Some(straight) = Self::consecutive(&self.cards) {
            return Some(Hand::Straight(straight));
        }

        if self.cards.first()?.rank == Rank::Two && self.cards.last()?.rank == Rank::Ace {
            let mut cards = self.cards.clone();
            let ace = cards.pop()?;
            cards.insert(0, ace);
            let straight: [Card; 5] = Self::consecutive(&cards)?;
            return Some(Hand::Straight(straight));
        }

        None
    }

    fn flush(&self) -> Option<Hand> {
        let flush: [Card; 5] = Self::same_suit(&self.cards)?;
        Some(Hand::Flush(flush))
    }

    fn full_house(&self) -> Option<Hand> {
        let mut cards = self.cards.clone();
        let three_of_a_kind: [Card; 3] = Self::same_rank(&cards)?;

        cards.retain(|card| card.rank != three_of_a_kind[0].rank);
        let pair: [Card; 2] = Self::same_rank(&cards)?;

        Some(Hand::FullHouse(three_of_a_kind, pair))
    }

    fn four_of_a_kind(&self) -> Option<Hand> {
        self.cards
            .windows(4)
            .find(|cards| cards.iter().all(|card| card.rank == cards[0].rank))
            .map(|cards| cards.to_owned().try_into().ok())
            .map(|cards| cards.map(Hand::FourOfAKind))
            .flatten()
    }

    fn straight_flush(&self) -> Option<Hand> {
        let flush: [Card; 5] = Self::same_suit(&self.cards)?;
        let deck = Deck {
            cards: flush.to_vec(),
        };
        if let Hand::Straight(straight) = deck.straight()? {
            return Some(Hand::StraightFlush(straight));
        }
        None
    }

    fn royal_flush(&self) -> Option<Hand> {
        if let Hand::StraightFlush(straight_flush) = self.straight_flush()? {
            if straight_flush[0].rank == Rank::Ten {
                return Some(Hand::RoyalFlush(straight_flush));
            }
        }
        None
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn try_from(s: &str) -> Option<Self> {
        use Rank::*;
        use Suit::*;

        let mut split_at = s.split(" ");
        let (rank, suit) = (split_at.next()?, split_at.next()?);

        let rank = match rank {
            "2" => Two,
            "3" => Three,
            "4" => Four,
            "5" => Five,
            "6" => Six,
            "7" => Seven,
            "8" => Eight,
            "9" => Nine,
            "10" => Ten,
            "J" => Jack,
            "Q" => Queen,
            "K" => King,
            "A" => Ace,
            _ => return None,
        };

        let suit = match suit {
            "♠" => Spades,
            "♥" => Hearts,
            "♦" => Diamonds,
            "♣" => Clubs,
            _ => return None,
        };

        Some(Card { rank, suit })
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Hand::*;
        match self {
            HighCard(card) => write!(f, "HighCard({})", card),
            Pair(cards) => write!(f, "Pair({} {})", cards[0], cards[1]),
            TwoPair(cards1, cards2) => write!(
                f,
                "TwoPair({} {} {} {})",
                cards1[0], cards1[1], cards2[0], cards2[1]
            ),
            ThreeOfAKind(cards) => {
                write!(f, "ThreeOfAKind({} {} {})", cards[0], cards[1], cards[2])
            }
            Straight(cards) => write!(
                f,
                "Straight({} {} {} {} {})",
                cards[0], cards[1], cards[2], cards[3], cards[4]
            ),
            Flush(cards) => write!(
                f,
                "Flush({} {} {} {} {})",
                cards[0], cards[1], cards[2], cards[3], cards[4]
            ),
            FullHouse(cards1, cards2) => write!(
                f,
                "FullHouse({} {} {} {} {})",
                cards1[0], cards1[1], cards1[2], cards2[0], cards2[1]
            ),
            FourOfAKind(cards) => write!(
                f,
                "FourOfAKind({} {} {} {})",
                cards[0], cards[1], cards[2], cards[3]
            ),
            StraightFlush(cards) => write!(
                f,
                "StraightFlush({} {} {} {} {})",
                cards[0], cards[1], cards[2], cards[3], cards[4]
            ),
            RoyalFlush(cards) => write!(
                f,
                "RoyalFlush({} {} {} {} {})",
                cards[0], cards[1], cards[2], cards[3], cards[4]
            ),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Rank::*;
        use Suit::*;
        let rank = match self.rank {
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
            Ace => "A",
        };
        let suit = match self.suit {
            Spades => "♠",
            Hearts => "♥",
            Diamonds => "♦",
            Clubs => "♣",
        };
        write!(f, "{} {}", rank, suit)
    }
}

impl<'a> Display for DeckFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cards = self.deck.cards.iter();
        let rows = *self.rows;

        while cards.len() > 0 {

            if let Some(card) = cards.next() {
                write!(f, "{}", card)?;
            }

            for _ in 1..rows {
                if let Some(card) = cards.next() {
                    let card = format!("{}", card);
                    write!(f, "{: >5}", card)?;
                }
            }

            if cards.len() > 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Deck { cards }
    }
}

impl From<Hand> for usize {
    fn from(hand: Hand) -> Self {
        use Hand::*;
        match hand {
            HighCard(_) => 0,
            Pair(_) => 1,
            TwoPair(_, _) => 2,
            ThreeOfAKind(_) => 3,
            Straight(_) => 4,
            Flush(_) => 5,
            FullHouse(_, _) => 6,
            FourOfAKind(_) => 7,
            StraightFlush(_) => 8,
            RoyalFlush(_) => 9,
        }
    }
}

impl From<usize> for Hand {
    fn from(idx: usize) -> Self {
        use Hand::*;
        let cards = [Card::default(); 5];

        match idx {
            0 => HighCard(cards[0]),
            1 => Pair([cards[0], cards[1]]),
            2 => TwoPair([cards[0], cards[1]], [cards[2], cards[3]]),
            3 => ThreeOfAKind([cards[0], cards[1], cards[2]]),
            4 => Straight(cards),
            5 => Flush(cards),
            6 => FullHouse([cards[0], cards[1], cards[2]], [cards[3], cards[4]]),
            7 => FourOfAKind([cards[0], cards[1], cards[2], cards[3]]),
            8 => StraightFlush(cards),
            9 => RoyalFlush(cards),
            _ => unreachable!(),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            rank: Rank::Two,
            suit: Suit::Spades,
        }
    }
}

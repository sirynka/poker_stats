mod code;
mod data;
mod test;

use crate::data::{Card, Deck, Hand, Rank, Suit};
use itertools::Itertools;
use std::time::Instant;
use strum::IntoEnumIterator;

fn _propability_to_win() {
    let players = 4;
    let iterations = 1_000_000;
    let timer = Instant::now();

    let mut wins_per_hand = vec![0; Hand::iter().count()];
    let mut losses_per_hand = vec![0; Hand::iter().count()];

    for _ in 0..iterations {
        let mut deck = Deck::french().shuffle();

        let table = deck.deal(5);

        let hands = (0..players).map(|_| deck.deal(2)).collect::<Vec<_>>();

        let mut hands: Vec<_> = hands
            .into_iter()
            .map(|hand| Deck::best_hand(&table, hand))
            .sorted()
            .collect();

        if let Some(winner) = hands.pop() {
            let idx: usize = winner.into();
            wins_per_hand[idx] += 1;
        }

        for hand in hands {
            let idx: usize = hand.into();
            losses_per_hand[idx] += 1;
        }
    }

    let hand_title = "Hand";
    let hand_probability_title = "Hand probability";
    let probability_to_win_title = "Probability to win";

    println!("Players: {}", players);

    println!(
        "{: <14}: {: >hp$}, {: >ptw$}",
        hand_title,
        hand_probability_title,
        probability_to_win_title,
        hp = hand_probability_title.len(),
        ptw = probability_to_win_title.len()
    );

    for (idx, (wins, loses)) in
        Iterator::zip(wins_per_hand.iter(), losses_per_hand.iter()).enumerate()
    {
        let hand_probability = *wins as f64 / iterations as f64 * 100.0;
        let probability_to_win = *wins as f64 / (*wins + *loses) as f64 * 100.0;

        let hand = Hand::from(idx);
        if let Some(hand) = format!("{}", hand).split("(").next() {
            println!(
                "{: <14}: {: >hp$.2}%, {: >ptw$.2}%",
                hand,
                hand_probability,
                probability_to_win,
                hp = hand_probability_title.len() - 1,
                ptw = probability_to_win_title.len() - 1
            );
        }
    }

    println!(
        "Simulated {:e} games in {:.2?}",
        iterations,
        timer.elapsed()
    );
}

fn _ways_to_improve_a_hand() {
    let mut deck = Deck::french().shuffle();

    let mut table = deck.deal(4);

    let hand = deck.deal(2);
    let best_hand = Deck::best_hand(&table, hand.clone());

    let cards_that_improve_hand: Vec<_> = deck
        .cards
        .iter()
        .filter_map(|card| {
            table.cards.push(*card);
            let possible_hands = Deck::all_hands(&table, hand.clone());
            table.cards.pop();

            let ace = Card::new(Rank::Ace, Suit::Clubs);
            let possible_hands: Vec<_> = possible_hands
                .into_iter()
                .filter(|hand| hand > &best_hand)
                .filter(|hand| hand > &Hand::TwoPair([ace, ace], [ace, ace]))
                .collect();

            if possible_hands.is_empty() {
                return None;
            }

            Some((card, possible_hands))
        })
        .sorted_by_key(|(_, hands)| *hands.first().unwrap())
        .collect();

    println!("Table({})", table.rows(&5));
    println!("Hand({}), {}", hand.rows(&2), best_hand);

    for (card, hands) in &cards_that_improve_hand {
        let card = format!("{}", card);
        print!("{: >4}: ", card);
        for hand in hands {
            print!("{}, ", hand);
        }
        println!();
    }

    println!("Better hands: {}", cards_that_improve_hand.len());
}

fn main() {
    // _propability_to_win();
    _ways_to_improve_a_hand();
}

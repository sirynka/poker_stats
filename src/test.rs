#[cfg(test)]
// ♠ ♥ ♦ ♣

mod test {
    use crate::data::{Card, Deck, Hand};

    fn card_parse(s: &str) -> Card {
        Card::try_from(s).unwrap()
    }

    #[test]
    pub fn highest_card() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("6 ♦"),
            card_parse("8 ♦"),
            card_parse("10 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::HighCard(card_parse("A ♣"));
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn pair() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("6 ♦"),
            card_parse("8 ♦"),
            card_parse("10 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("Q ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::Pair([card_parse("Q ♥"), card_parse("Q ♣")]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn two_pairs() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("6 ♦"),
            card_parse("8 ♦"),
            card_parse("8 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("Q ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::TwoPair(
            [card_parse("Q ♥"), card_parse("Q ♣")],
            [card_parse("8 ♥"), card_parse("8 ♦")],
        );
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn three_pairs() {
        let table = Deck::from(vec![
            card_parse("4 ♠"),
            card_parse("4 ♦"),
            card_parse("8 ♠"),
            card_parse("8 ♦"),
            card_parse("9 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("Q ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::TwoPair(
            [card_parse("Q ♥"), card_parse("Q ♣")],
            [card_parse("8 ♠"), card_parse("8 ♦")],
        );
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn three_of_a_kind() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("8 ♠"),
            card_parse("8 ♦"),
            card_parse("8 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected =
            Hand::ThreeOfAKind([card_parse("8 ♠"), card_parse("8 ♥"), card_parse("8 ♦")]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn straight() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("3 ♠"),
            card_parse("4 ♦"),
            card_parse("5 ♦"),
            card_parse("6 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("7 ♥"), card_parse("8 ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::Straight([
            card_parse("4 ♦"),
            card_parse("5 ♦"),
            card_parse("6 ♥"),
            card_parse("7 ♥"),
            card_parse("8 ♣"),
        ]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn straight_with_ace() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("3 ♠"),
            card_parse("4 ♦"),
            card_parse("5 ♦"),
            card_parse("7 ♥"),
        ]);

        let hand = Deck::from(vec![card_parse("8 ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::Straight([
            card_parse("A ♣"),
            card_parse("2 ♠"),
            card_parse("3 ♠"),
            card_parse("4 ♦"),
            card_parse("5 ♦"),
        ]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn flush() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("6 ♠"),
            card_parse("8 ♠"),
            card_parse("10 ♠"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::Flush([
            card_parse("2 ♠"),
            card_parse("4 ♠"),
            card_parse("6 ♠"),
            card_parse("8 ♠"),
            card_parse("10 ♠"),
        ]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn full_house() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("2 ♦"),
            card_parse("2 ♥"),
            card_parse("8 ♠"),
            card_parse("8 ♦"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::FullHouse(
            [card_parse("2 ♠"), card_parse("2 ♥"), card_parse("2 ♦")],
            [card_parse("8 ♠"), card_parse("8 ♦")],
        );
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn four_of_a_kind() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("2 ♦"),
            card_parse("2 ♥"),
            card_parse("2 ♣"),
            card_parse("8 ♦"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::FourOfAKind([
            card_parse("2 ♠"),
            card_parse("2 ♥"),
            card_parse("2 ♦"),
            card_parse("2 ♣"),
        ]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn straight_flush() {
        let table = Deck::from(vec![
            card_parse("2 ♠"),
            card_parse("3 ♠"),
            card_parse("4 ♠"),
            card_parse("5 ♠"),
            card_parse("6 ♠"),
        ]);

        let hand = Deck::from(vec![card_parse("Q ♥"), card_parse("A ♣")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::StraightFlush([
            card_parse("2 ♠"),
            card_parse("3 ♠"),
            card_parse("4 ♠"),
            card_parse("5 ♠"),
            card_parse("6 ♠"),
        ]);
        assert_eq!(hand, expected);
    }

    #[test]
    pub fn royal_flush() {
        let table = Deck::from(vec![
            card_parse("10 ♠"),
            card_parse("J ♠"),
            card_parse("Q ♠"),
            card_parse("K ♠"),
            card_parse("A ♠"),
        ]);

        let hand = Deck::from(vec![card_parse("9 ♠"), card_parse("8 ♠")]);

        let hand = Deck::best_hand(&table, hand);
        let expected = Hand::RoyalFlush([
            card_parse("10 ♠"),
            card_parse("J ♠"),
            card_parse("Q ♠"),
            card_parse("K ♠"),
            card_parse("A ♠"),
        ]);
        assert_eq!(hand, expected);
    }
}

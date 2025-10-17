use crate::blackjack::types::Card;
use std::fmt;

/// Represents a hand of cards with intelligent scoring
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new empty hand
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    /// Adds a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Returns a reference to the cards in the hand
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Returns the number of cards in the hand
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the hand is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Calculates the best possible score for this hand
    ///
    /// Aces can be worth 1 or 11. This method finds the highest score
    /// that doesn't exceed 21, or the lowest score if all scores bust.
    pub fn score(&self) -> u8 {
        let mut total = 0u8;
        let mut ace_count = 0;

        // First, count all cards with Aces as 11
        for card in &self.cards {
            total += card.rank.value();
            if card.rank.is_ace() {
                ace_count += 1;
            }
        }

        // If we're over 21 and have aces, convert them to 1 until we're under 21
        while total > 21 && ace_count > 0 {
            total -= 10; // Convert an Ace from 11 to 1 (difference of 10)
            ace_count -= 1;
        }

        total
    }

    /// Returns true if this hand is a bust (score > 21)
    pub fn is_bust(&self) -> bool {
        self.score() > 21
    }

    /// Returns true if this hand is a blackjack (21 with exactly 2 cards)
    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.score() == 21
    }

    /// Clears all cards from the hand
    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "Empty hand");
        }

        let cards_str: Vec<String> = self.cards.iter().map(|c| c.to_string()).collect();
        write!(f, "{} (score: {})", cards_str.join(" "), self.score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blackjack::types::{Rank, Suit};

    #[test]
    fn test_empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.score(), 0);
        assert_eq!(hand.len(), 0);
        assert!(hand.is_empty());
    }

    #[test]
    fn test_simple_score() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Five));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Seven));
        assert_eq!(hand.score(), 12);
    }

    #[test]
    fn test_ace_as_eleven() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Nine));
        assert_eq!(hand.score(), 20); // Ace counts as 11
    }

    #[test]
    fn test_ace_as_one() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::King));
        hand.add_card(Card::new(Suit::Clubs, Rank::Five));
        assert_eq!(hand.score(), 16); // Ace counts as 1 (11+10+5=26, convert ace: 1+10+5=16)
    }

    #[test]
    fn test_multiple_aces() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Ace));
        hand.add_card(Card::new(Suit::Clubs, Rank::Nine));
        assert_eq!(hand.score(), 21); // 11 + 1 + 9 = 21
    }

    #[test]
    fn test_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::King));
        assert!(hand.is_blackjack());
        assert_eq!(hand.score(), 21);
    }

    #[test]
    fn test_not_blackjack_three_cards() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Seven));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Seven));
        hand.add_card(Card::new(Suit::Clubs, Rank::Seven));
        assert!(!hand.is_blackjack());
        assert_eq!(hand.score(), 21);
    }

    #[test]
    fn test_bust() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::King));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Queen));
        hand.add_card(Card::new(Suit::Clubs, Rank::Five));
        assert!(hand.is_bust());
        assert_eq!(hand.score(), 25);
    }
}

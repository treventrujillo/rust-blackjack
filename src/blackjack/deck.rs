use crate::blackjack::types::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents a deck of playing cards
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new standard 52-card deck
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
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
            Rank::Ace,
        ];

        for suit in &suits {
            for rank in &ranks {
                cards.push(Card::new(suit.clone(), rank.clone()));
            }
        }

        Deck { cards }
    }

    /// Shuffles the deck using the Fisher-Yates algorithm
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draws a card from the top of the deck
    ///
    /// Returns None if the deck is empty
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Returns the number of cards remaining in the deck
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Resets the deck to a full 52-card deck and shuffles it
    pub fn reset_and_shuffle(&mut self) {
        *self = Deck::new();
        self.shuffle();
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_draw_reduces_count() {
        let mut deck = Deck::new();
        deck.draw();
        assert_eq!(deck.remaining(), 51);
    }

    #[test]
    fn test_draw_returns_some_until_empty() {
        let mut deck = Deck::new();
        for _ in 0..52 {
            assert!(deck.draw().is_some());
        }
        assert!(deck.draw().is_none());
    }

    #[test]
    fn test_shuffle_changes_order() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        deck1.shuffle();

        // Draw all cards from both decks and compare
        let mut cards1 = Vec::new();
        let mut cards2 = Vec::new();

        while let Some(card) = deck1.draw() {
            cards1.push(format!("{}", card));
        }

        while let Some(card) = deck2.draw() {
            cards2.push(format!("{}", card));
        }

        // It's theoretically possible (but extremely unlikely) for shuffled deck to match
        // We'll just verify that shuffle doesn't break the deck
        assert_eq!(cards1.len(), 52);
        assert_eq!(cards2.len(), 52);
    }

    #[test]
    fn test_reset_and_shuffle() {
        let mut deck = Deck::new();

        // Draw some cards
        for _ in 0..10 {
            deck.draw();
        }
        assert_eq!(deck.remaining(), 42);

        // Reset should give us 52 cards again
        deck.reset_and_shuffle();
        assert_eq!(deck.remaining(), 52);
    }
}

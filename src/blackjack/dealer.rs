use crate::blackjack::hand::Hand;
use crate::blackjack::types::Card;

/// Represents the dealer in the game
#[derive(Debug)]
pub struct Dealer {
    hand: Hand,
}

impl Dealer {
    /// Creates a new dealer with an empty hand
    pub fn new() -> Self {
        Dealer { hand: Hand::new() }
    }

    /// Returns a reference to the dealer's hand
    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    /// Returns a mutable reference to the dealer's hand
    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    /// Adds a card to the dealer's hand
    pub fn receive_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// Clears the dealer's hand (for starting a new round)
    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    /// Returns the current score of the dealer's hand
    pub fn score(&self) -> u8 {
        self.hand.score()
    }

    /// Returns true if the dealer has busted
    pub fn is_bust(&self) -> bool {
        self.hand.is_bust()
    }

    /// Returns true if the dealer has blackjack
    pub fn is_blackjack(&self) -> bool {
        self.hand.is_blackjack()
    }

    /// Determines if the dealer should hit based on standard blackjack rules
    /// Dealer must hit on 16 or less, and stand on 17 or more
    pub fn should_hit(&self) -> bool {
        self.hand.score() < 17
    }
}

impl Default for Dealer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blackjack::types::{Rank, Suit};

    #[test]
    fn test_new_dealer_empty_hand() {
        let dealer = Dealer::new();
        assert_eq!(dealer.score(), 0);
        assert_eq!(dealer.hand().len(), 0);
    }

    #[test]
    fn test_receive_card() {
        let mut dealer = Dealer::new();
        dealer.receive_card(Card::new(Suit::Hearts, Rank::King));
        assert_eq!(dealer.score(), 10);
    }

    #[test]
    fn test_should_hit() {
        let mut dealer = Dealer::new();
        dealer.receive_card(Card::new(Suit::Hearts, Rank::Ten));
        dealer.receive_card(Card::new(Suit::Diamonds, Rank::Six));
        assert!(dealer.should_hit()); // 16, should hit

        dealer.receive_card(Card::new(Suit::Clubs, Rank::Two));
        assert!(!dealer.should_hit()); // 18, should stand
    }

    #[test]
    fn test_clear_hand() {
        let mut dealer = Dealer::new();
        dealer.receive_card(Card::new(Suit::Hearts, Rank::King));
        dealer.clear_hand();
        assert_eq!(dealer.score(), 0);
        assert_eq!(dealer.hand().len(), 0);
    }
}

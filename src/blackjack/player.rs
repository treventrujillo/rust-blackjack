use crate::blackjack::hand::Hand;
use crate::blackjack::types::Card;

/// Represents a player in the game
#[derive(Debug)]
pub struct Player {
    hand: Hand,
}

impl Player {
    /// Creates a new player with an empty hand
    pub fn new() -> Self {
        Player { hand: Hand::new() }
    }

    /// Returns a reference to the player's hand
    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    /// Returns a mutable reference to the player's hand
    #[allow(dead_code)]
    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    /// Adds a card to the player's hand
    pub fn receive_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// Clears the player's hand (for starting a new round)
    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    /// Returns the current score of the player's hand
    pub fn score(&self) -> u8 {
        self.hand.score()
    }

    /// Returns true if the player has busted
    pub fn is_bust(&self) -> bool {
        self.hand.is_bust()
    }

    /// Returns true if the player has blackjack
    pub fn is_blackjack(&self) -> bool {
        self.hand.is_blackjack()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blackjack::types::{Rank, Suit};

    #[test]
    fn test_new_player_empty_hand() {
        let player = Player::new();
        assert_eq!(player.score(), 0);
        assert_eq!(player.hand().len(), 0);
    }

    #[test]
    fn test_receive_card() {
        let mut player = Player::new();
        player.receive_card(Card::new(Suit::Hearts, Rank::King));
        assert_eq!(player.score(), 10);
    }

    #[test]
    fn test_clear_hand() {
        let mut player = Player::new();
        player.receive_card(Card::new(Suit::Hearts, Rank::King));
        player.clear_hand();
        assert_eq!(player.score(), 0);
        assert_eq!(player.hand().len(), 0);
    }

    #[test]
    fn test_is_bust() {
        let mut player = Player::new();
        player.receive_card(Card::new(Suit::Hearts, Rank::King));
        player.receive_card(Card::new(Suit::Diamonds, Rank::Queen));
        player.receive_card(Card::new(Suit::Clubs, Rank::Five));
        assert!(player.is_bust());
    }

    #[test]
    fn test_is_blackjack() {
        let mut player = Player::new();
        player.receive_card(Card::new(Suit::Hearts, Rank::Ace));
        player.receive_card(Card::new(Suit::Diamonds, Rank::King));
        assert!(player.is_blackjack());
    }
}

use crate::blackjack::dealer::Dealer;
use crate::blackjack::deck::Deck;
use crate::blackjack::player::Player;
use crate::blackjack::types::{Action, GameOutcome};

/// The main game engine that orchestrates blackjack gameplay
pub struct GameEngine {
    deck: Deck,
    player: Player,
    dealer: Dealer,
    outcome: GameOutcome,
}

impl GameEngine {
    /// Creates a new game with a shuffled deck
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        GameEngine {
            deck,
            player: Player::new(),
            dealer: Dealer::new(),
            outcome: GameOutcome::InProgress,
        }
    }

    /// Returns a reference to the player
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Returns a reference to the dealer
    pub fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    /// Returns the current game outcome
    pub fn outcome(&self) -> GameOutcome {
        self.outcome
    }

    /// Starts a new round by dealing initial cards
    pub fn deal_initial_cards(&mut self) {
        // Clear previous hands
        self.player.clear_hand();
        self.dealer.clear_hand();
        self.outcome = GameOutcome::InProgress;

        // Deal alternating cards: player, dealer, player, dealer
        if let Some(card) = self.deck.draw() {
            self.player.receive_card(card);
        }
        if let Some(card) = self.deck.draw() {
            self.dealer.receive_card(card);
        }
        if let Some(card) = self.deck.draw() {
            self.player.receive_card(card);
        }
        if let Some(card) = self.deck.draw() {
            self.dealer.receive_card(card);
        }

        // Check for immediate blackjacks
        if self.player.is_blackjack() && self.dealer.is_blackjack() {
            self.outcome = GameOutcome::Push;
        } else if self.player.is_blackjack() {
            self.outcome = GameOutcome::PlayerWins;
        } else if self.dealer.is_blackjack() {
            self.outcome = GameOutcome::DealerWins;
        }
    }

    /// Executes a player action and returns the new game state
    pub fn execute_action(&mut self, action: Action) -> GameOutcome {
        if self.outcome != GameOutcome::InProgress {
            return self.outcome;
        }

        match action {
            Action::Hit => {
                // Player draws a card
                if let Some(card) = self.deck.draw() {
                    self.player.receive_card(card);
                }

                // Check if player busts
                if self.player.is_bust() {
                    self.outcome = GameOutcome::DealerWins;
                }
            }
            Action::Stand => {
                // Player stands, dealer plays their hand
                self.play_dealer_hand();
            }
            Action::DoubleDown => {
                // Player draws exactly one more card, then dealer plays
                if let Some(card) = self.deck.draw() {
                    self.player.receive_card(card);
                }

                if self.player.is_bust() {
                    self.outcome = GameOutcome::DealerWins;
                } else {
                    self.play_dealer_hand();
                }
            }
        }

        self.outcome
    }

    /// Dealer plays their hand according to standard rules
    fn play_dealer_hand(&mut self) {
        // Dealer hits until they reach 17 or higher
        while self.dealer.should_hit() {
            if let Some(card) = self.deck.draw() {
                self.dealer.receive_card(card);
            } else {
                // Out of cards - shouldn't happen in normal play
                break;
            }
        }

        // Determine outcome
        if self.dealer.is_bust() {
            self.outcome = GameOutcome::PlayerWins;
        } else {
            let player_score = self.player.score();
            let dealer_score = self.dealer.score();

            self.outcome = if player_score > dealer_score {
                GameOutcome::PlayerWins
            } else if dealer_score > player_score {
                GameOutcome::DealerWins
            } else {
                GameOutcome::Push
            };
        }
    }

    /// Resets the deck (useful when running low on cards)
    pub fn reset_deck(&mut self) {
        self.deck.reset_and_shuffle();
    }

    /// Returns the number of cards remaining in the deck
    pub fn cards_remaining(&self) -> usize {
        self.deck.remaining()
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameEngine::new();
        assert_eq!(game.outcome(), GameOutcome::InProgress);
        assert_eq!(game.player().score(), 0);
        assert_eq!(game.dealer().score(), 0);
    }

    #[test]
    fn test_deal_initial_cards() {
        let mut game = GameEngine::new();
        game.deal_initial_cards();

        assert_eq!(game.player().hand().len(), 2);
        assert_eq!(game.dealer().hand().len(), 2);
    }

    #[test]
    fn test_player_hit_and_bust() {
        let mut game = GameEngine::new();
        game.deal_initial_cards();

        // Keep hitting until player busts (this will eventually happen)
        let mut hit_count = 0;
        while game.outcome() == GameOutcome::InProgress && hit_count < 10 {
            game.execute_action(Action::Hit);
            hit_count += 1;

            if game.player().is_bust() {
                assert_eq!(game.outcome(), GameOutcome::DealerWins);
                break;
            }
        }
    }

    #[test]
    fn test_player_stand() {
        let mut game = GameEngine::new();
        game.deal_initial_cards();

        if game.outcome() == GameOutcome::InProgress {
            game.execute_action(Action::Stand);
            // After standing, game should be decided
            assert_ne!(game.outcome(), GameOutcome::InProgress);
        }
    }

    #[test]
    fn test_double_down() {
        let mut game = GameEngine::new();
        game.deal_initial_cards();

        let initial_cards = game.player().hand().len();

        if game.outcome() == GameOutcome::InProgress {
            game.execute_action(Action::DoubleDown);
            // Player should have exactly one more card
            assert!(game.player().hand().len() == initial_cards + 1);
            // Game should be decided
            assert_ne!(game.outcome(), GameOutcome::InProgress);
        }
    }

    #[test]
    fn test_reset_deck() {
        let mut game = GameEngine::new();

        // Draw many cards
        for _ in 0..20 {
            game.deck.draw();
        }

        let remaining = game.cards_remaining();
        assert_eq!(remaining, 32);

        game.reset_deck();
        assert_eq!(game.cards_remaining(), 52);
    }
}

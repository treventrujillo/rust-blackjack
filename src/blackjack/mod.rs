// Core type definitions
mod types;

// Game components
mod deck;
mod hand;
mod player;
mod dealer;

// Game logic
mod game;

// I/O utilities
mod io;

// Re-export public API
pub use game::GameEngine;
pub use types::{Action, GameOutcome};

use io::{ask_play_again, display_game_state, display_outcome, display_welcome, get_player_action};

/// Main entry point for playing blackjack
pub fn play() {
    display_welcome();

    let mut game = GameEngine::new();
    let mut keep_playing = true;

    while keep_playing {
        // Check if deck is running low, reset if needed
        if game.cards_remaining() < 15 {
            println!("\n[Shuffling new deck...]");
            game.reset_deck();
        }

        // Deal initial cards
        game.deal_initial_cards();

        // Show initial game state (hide one dealer card)
        display_game_state(game.player(), game.dealer(), true);

        // Check for immediate blackjacks
        if game.outcome() != GameOutcome::InProgress {
            display_outcome(game.outcome(), game.player(), game.dealer());
            keep_playing = ask_play_again();
            continue;
        }

        // Player's turn
        loop {
            // Only allow double down on first action (when player has exactly 2 cards)
            let allow_double_down = game.player().hand().len() == 2;

            match get_player_action(allow_double_down) {
                Ok(action) => {
                    let outcome = game.execute_action(action);

                    if action == Action::Stand || action == Action::DoubleDown {
                        // Game is over, show final results
                        display_outcome(outcome, game.player(), game.dealer());
                        break;
                    } else {
                        // Player hit, show updated game state
                        display_game_state(game.player(), game.dealer(), true);

                        if outcome != GameOutcome::InProgress {
                            // Player busted
                            display_outcome(outcome, game.player(), game.dealer());
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        keep_playing = ask_play_again();
    }

    println!("\nThanks for playing!");
}

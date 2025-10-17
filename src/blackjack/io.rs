use crate::blackjack::dealer::Dealer;
use crate::blackjack::player::Player;
use crate::blackjack::types::{Action, GameOutcome};
use std::io::{self, Write};

/// Displays the current game state to the user
pub fn display_game_state(player: &Player, dealer: &Dealer, hide_dealer_card: bool) {
    println!("\n{}", "=".repeat(50));

    if hide_dealer_card {
        // Show only one dealer card (first card is hidden)
        let dealer_cards = dealer.hand().cards();
        if !dealer_cards.is_empty() {
            println!("Dealer's hand: [Hidden] {}", dealer_cards[dealer_cards.len() - 1]);
        }
    } else {
        // Show full dealer hand
        println!("Dealer's hand: {}", dealer.hand());
    }

    println!("Player's hand: {}", player.hand());
    println!("{}", "=".repeat(50));
}

/// Prompts the user to select an action
pub fn get_player_action(allow_double_down: bool) -> Result<Action, String> {
    loop {
        println!("\nWhat would you like to do?");
        println!("  1. Hit");
        println!("  2. Stand");
        if allow_double_down {
            println!("  3. Double Down");
        }
        print!("Enter your choice (1-{}): ", if allow_double_down { 3 } else { 2 });
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        match input.trim() {
            "1" => return Ok(Action::Hit),
            "2" => return Ok(Action::Stand),
            "3" if allow_double_down => return Ok(Action::DoubleDown),
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

/// Displays the final game outcome
pub fn display_outcome(outcome: GameOutcome, player: &Player, dealer: &Dealer) {
    println!("\n{}", "=".repeat(50));
    println!("GAME OVER");
    println!("{}", "=".repeat(50));
    println!("Player's final hand: {}", player.hand());
    println!("Dealer's final hand: {}", dealer.hand());
    println!("\n{}", outcome);
    println!("{}", "=".repeat(50));
}

/// Asks the user if they want to play another round
pub fn ask_play_again() -> bool {
    loop {
        print!("\nPlay another round? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

/// Displays a welcome message
pub fn display_welcome() {
    println!("\n{}", "=".repeat(50));
    println!("Welcome to Blackjack!");
    println!("{}", "=".repeat(50));
    println!("\nRules:");
    println!("  - Get as close to 21 as possible without going over");
    println!("  - Aces count as 1 or 11 (whichever is better)");
    println!("  - Face cards (J, Q, K) count as 10");
    println!("  - Dealer must hit on 16 or less, stand on 17 or more");
    println!("{}", "=".repeat(50));
}

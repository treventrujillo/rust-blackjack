mod dealer;
mod deck;

use std::error::Error;
use std::io;
use crate::blackjack::dealer::Dealer;

pub fn play() -> Result<(), Box<dyn Error>> {
    println!("~*~*~*~ IT'S TIME TO GAMBLE ~*~*~*~");

    let mut dealer = Dealer::new();
    let mut player_hand = dealer.deal_new_hand();

    let mut game_over = false;

    while game_over != true {
        let choice = get_choice().unwrap();

        let outcome = match choice {
            1 => stand(&mut dealer),
            2 => Ok(0),
            3 => Ok(0),
            _ => panic!("Invalid choice.")
        };

        let resolved_outcome = outcome.expect("Failed to retrieve outcome.");

        // 0: Round continues
        // 1: Dealer wins
        // 2: Player wins
        // TODO: Make this integer an enum for cohesion
        if resolved_outcome == 0 {
            println!("Round continues.");
        } else if resolved_outcome == 1 {
            println!("Dealer wins! Round over.");
            game_over = true;
        } else if resolved_outcome == 2 {
            println!("Player wins! Round over.");
            game_over = true;
        } else {
            panic!("Invalid outcome {}", resolved_outcome);
        }
    }

    Ok(())
}

fn stand(dealer: &mut Dealer) -> Result<u8, Box<dyn Error>> {
    dealer.deal_self().expect("Failed to deal self!");

    let dealer_hand = dealer.hand.as_ref().expect("No hand found!");
    let score = dealer_hand.get_score();

    println!("\nDealer's hand: {:?}", score);
    dealer.print_hand(&dealer_hand.cards);

    if score == 21 {
        Ok(1)
    } else if score > 21 {
        Ok(2)
    } else {
        Ok(0)
    }
}

fn get_choice() -> Result<u8, Box<dyn Error>> {
    println!("\nStand: 1\nHit: 2\nDouble: 3\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to parse user input.");

    match input.trim().parse::<u8>() {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(e))
    }
}

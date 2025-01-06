use std::error::Error;
use std::fmt::Debug;
use std::io;
use crate::dealer::Dealer;

pub fn play() -> Result<(), Box<dyn Error>> {
    println!("~*~*~*~ IT'S TIME TO GAMBLE ~*~*~*~");

    let mut dealer = Dealer::new();
    let (mut dealer_hand, mut player_hand) = dealer.deal_new_hand();

    loop {
        let choice = get_choice().unwrap();

        match choice {
            1 => {
                dealer.deal(&mut dealer_hand);
                println!("\nDealer's hand: {}", &dealer_hand.get_score());
                dealer.print_hand(&dealer_hand.cards)
            },
            2 => {},
            3 => {},
            _ => panic!("Invalid choice.")
        }
    }

    Ok(())
}

fn get_choice() -> Result<u8, Box<dyn Error>> {
    println!("Stand: 1\nHit: 2\nDouble: 3\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to parse user input.");

    match input.trim().parse::<u8>() {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(e))
    }
}

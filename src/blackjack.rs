use std::error::Error;
use std::fmt::Debug;
use std::io;
use crate::dealer::Dealer;

pub fn play() -> Result<(), Box<dyn Error>> {
    let mut dealer = Dealer::new();
    let (mut dealer_hand, mut player_hand) = dealer.deal_new_hand();

    println!("Stand: 1\nHit: 2\nDouble: 3");

    let mut input = String::new();
    let choice = io::stdin().read_line(&mut input).unwrap();

    Ok(())
}

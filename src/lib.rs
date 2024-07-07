mod dealer;

use std::fmt::Debug;
use dealer::{
    Dealer,
    Rank,
    Suit,
};

pub fn run () {
    let suits = Suit::all_suits();
    let ranks = Rank::all_ranks();

    let mut dealer = Dealer::new(&suits, &ranks);

    println!("Un-shuffled deck:");
    print_collection(&dealer.deck);

    // Shuffle the deck three times
    for _ in 0..3 {
        println!("Shuffling deck...");
        dealer.shuffle_deck();
    }

    println!("Shuffled deck: ");
    print_collection(&dealer.deck);
}

fn print_collection<T: Debug>(collection: &[T]) {
    for element in collection {
        println!("{:?}", element);
    }
}
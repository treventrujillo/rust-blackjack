mod dealer;

use std::fmt::Debug;
use dealer::{
    Dealer,
};

pub fn run() {
    let mut dealer = Dealer::new();
    dealer.shuffle_deck();
    print_collection(&dealer.get_cards());
}

fn print_collection<T: Debug>(collection: &[T]) {
    for element in collection {
        println!("{:?}", element);
    }
}

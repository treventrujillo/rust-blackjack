use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;
use crate::dealer::{Card, Dealer, Rank};

const MAX_HAND_SIZE: usize = 10;

struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: Vec::with_capacity(MAX_HAND_SIZE),
        }
    }

    fn get_score(&self) -> u32 {
        let mut ranks = Vec::with_capacity(MAX_HAND_SIZE);
        for card in &self.cards {
            ranks.push(resolve_rank(&card.rank))
        }
        ranks.iter().sum()
    }
}

pub fn play() -> Result<(), Box<dyn Error>> {
    let mut dealer = Dealer::new();

    let mut dealer_hand = Hand::new();
    let mut player_hand = Hand::new();

    player_hand.cards.push(dealer.deck.cards.pop().unwrap());
    dealer_hand.cards.push(dealer.deck.cards.pop().unwrap());

    println!("Dealer's hand: {:?}", &dealer_hand.get_score());
    print_collection(&dealer_hand.cards);

    dealer_hand.cards.push(dealer.deck.cards.pop().unwrap());
    player_hand.cards.push(dealer.deck.cards.pop().unwrap());

    println!("Player hand: {:?}", &player_hand.get_score());
    print_collection(&player_hand.cards);

    Ok(())
}

fn print_collection<T: Debug>(collection: &[T]) {
    for element in collection {
        println!("{:?}", element);
    }
}

fn resolve_rank(rank: &Rc<Rank>) -> u32 {
    match &**rank {
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten => 10,
        Rank::Jack => 10,
        Rank::Queen => 10,
        Rank::King => 10,
        Rank::Ace => 11, // we'll have to handle aces specially
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_rank() {
        let mut ranks = Vec::with_capacity(10);
        for rank in &vec![Rc::new(Rank::Jack), Rc::new(Rank::Ace)] {
            ranks.push(resolve_rank(rank));
        }

        assert_eq!(ranks, vec![10, 11])
    }
}
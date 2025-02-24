use std::cmp;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

use rand::{Rng, thread_rng};
use crate::blackjack::deck::*;

#[derive(Debug)]
pub struct Dealer {
    pub deck: Deck,
    pub hand: Option<Hand>,
}

impl Dealer {
    pub fn new() -> Self {
        let mut deck = Deck::build();

        // Shuffle three times initially
        for _ in 0..3 {
            Self::shuffle(&mut deck.cards);
        }

        Dealer { deck, hand: None }
    }

    fn shuffle(deck: &mut Vec<Card>) {
        // Iterate over the cards from the last to the first.
        // For each card at index i, generate a random index j such that 0 <= j <= i.
        // Swap the card at indices i and j.
        let size = deck.len();
        let mut rng = thread_rng();
        deck.reverse();
        for i in 0..size {
            // Initialize to size that will be regenerated
            let mut random_index = 99;
            while random_index >= i && random_index > size {
                random_index = rng.gen_range(1..=size);
            }

            let random_index = cmp::min(random_index, deck.len() - 2);
            let index = cmp::min(i, deck.len() - 2);

            let first_card = deck.remove(i);
            let second_card = deck.remove(random_index);

            deck.insert(index, second_card);
            deck.insert(random_index, first_card);
        }
    }

    // Deals new hand for self and player,
    // returns player hand.
    pub fn deal_new_hand(&mut self) -> Hand {
        let mut dealer_hand = Hand::new();
        let mut player_hand = Hand::new();

        player_hand.cards.push(self.pop_card_from_deck());
        dealer_hand.cards.push(self.pop_card_from_deck());

        println!("Dealer's hand: {:?}", &dealer_hand.get_score());
        self.print_hand(&dealer_hand.cards);

        player_hand.cards.push(self.pop_card_from_deck());
        dealer_hand.cards.push(self.pop_card_from_deck());

        println!("Player's hand: {:?}", &dealer_hand.get_score());
        self.print_hand(&dealer_hand.cards);

        self.hand = Some(dealer_hand);

        player_hand
    }

    pub fn deal_self(&mut self) -> Result<(), Box<dyn Error>> {
        let cards = &mut self.deck.cards;
        let _ = &self.hand.as_mut().expect("No hand to deal to!")
            .cards.push(cards.pop().unwrap());
        Ok(())
    }

    fn pop_card_from_deck(&mut self) -> Card {
        self.deck.cards.pop().unwrap()
    }

    pub fn print_hand<T: Debug>(&self, collection: &[T]) {
        for element in collection {
            println!("{:?}", element);
        }
        print_newline()
    }
}
fn print_newline() {
    println!("\n")
}

pub fn resolve_rank(rank: &Rc<Rank>) -> u32 {
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
        Rank::Ace => 11, // TODO: handle aces as either 1 or 11
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;

    #[test]
    fn test_build_deck() {
        let deck = Deck::build();
        assert_eq!(&Deck::DECK_SIZE, &deck.cards.len());
    }

    #[test]
    fn test_create_dealer() {
        assert!(matches!(Some(Dealer::new()), Some(_)));
    }

    #[test]
    fn test_resolve_rank() {
        let mut ranks = Vec::with_capacity(10);
        for rank in &vec![Rc::new(Rank::Jack), Rc::new(Rank::Ace)] {
            ranks.push(resolve_rank(rank));
        }

        assert_eq!(ranks, vec![10, 11])
    }
}

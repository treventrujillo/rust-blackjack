use rand::{Rng, RngCore, thread_rng};
use std::cmp;
use std::error::Error;

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all_suits() -> Vec<Suit> {
        vec![
            Suit::Hearts, Suit::Diamonds,
            Suit::Clubs, Suit::Spades,
        ]
    }
}

#[derive(Debug)]
pub enum Rank {
    Two, Three, Four, Five, Six,
    Seven, Eight, Nine, Ten, Jack,
    Queen, King, Ace,
}

impl Rank {
    pub fn all_ranks() -> Vec<Rank> {
        use Rank::*;
        vec![
            Two, Three, Four, Five, Six,
            Seven, Eight, Nine, Ten, Jack,
            Queen, King, Ace,
        ]
    }
}

#[derive(Debug)]
pub struct Card<'a > {
    suit: &'a Suit,
    rank: &'a Rank,
}

#[derive(Debug)]
pub struct Dealer<'a> {
    pub deck: Vec<Card<'a>>,
}

impl Dealer<'_> {
    pub fn new<'a>(suits: &'a [Suit], ranks: &'a [Rank]) -> Dealer<'a> {
        Dealer { deck: Self::create_deck(suits, ranks) }
    }

    fn create_deck<'a>(suits: &'a [Suit], ranks: &'a [Rank]) -> Vec<Card<'a>> {
        let mut deck = Vec::new();

        for suit in suits {
            for rank in ranks {
                deck.push(
                  Card { suit, rank }
                );
            }
        }

        deck
    }

    pub fn shuffle_deck(&mut self) {
        // Iterate over the vector from the last element to the first.
        // For each element at index i, generate a random index j such that 0 <= j <= i.
        // Swap the elements at indices i and j.
        let deck = &mut self.deck;
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
}

#[cfg(test)]
mod tests {
    use crate::dealer::{Dealer, Rank, Suit};

    // Test that shuffle works with no panics
    #[test]
    fn test_shuffle() {
        let suits = Suit::all_suits();
        let ranks = Rank::all_ranks();
        let mut dealer = Dealer::new(&suits, &ranks);
        let size = &dealer.deck.len();

        dealer.shuffle_deck();

        assert_eq!(size, &dealer.deck.len())
    }
}

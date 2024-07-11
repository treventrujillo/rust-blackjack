use rand::{Rng, thread_rng};
use std::cmp;
use std::rc::Rc;

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all_suits() -> Vec<Rc<Suit>> {
        vec![
            Rc::new(Suit::Hearts), Rc::new(Suit::Diamonds),
            Rc::new(Suit::Clubs), Rc::new(Suit::Spades),
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
    pub fn all_ranks() -> Vec<Rc<Rank>> {
        use Rank::*;
        vec![
            Rc::new(Two), Rc::new(Three), Rc::new(Four), Rc::new(Five), Rc::new(Six),
            Rc::new(Seven), Rc::new(Eight), Rc::new(Nine), Rc::new(Ten), Rc::new(Jack),
            Rc::new(Queen), Rc::new(King), Rc::new(Ace),
        ]
    }
}

#[derive(Debug)]
pub struct Card {
    suit: Rc<Suit>,
    pub rank: Rc<Rank>,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    const DECK_SIZE: usize = 52;
    pub fn build() -> Self {
        let mut cards = Vec::with_capacity(Deck::DECK_SIZE);

        let suits = &Suit::all_suits();
        let ranks = &Rank::all_ranks();

        for suit in suits {
            for rank in ranks {
                cards.push(
                    Card {
                        suit: Rc::clone(suit),
                        rank: Rc::clone(rank),
                    }
                );
            }
        }

        Deck { cards }
    }
}

#[derive(Debug)]
pub struct Dealer {
    pub deck: Deck,
}

impl Dealer {
    pub fn new() -> Self {
        let mut deck = Deck::build();

        // Shuffle three times initially
        for _ in 0..3 {
            Self::shuffle(&mut deck.cards);
        }

        Dealer { deck }
    }

    pub fn shuffle_deck(&mut self) {
        Self::shuffle(&mut self.deck.cards);
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
}

#[cfg(test)]
mod tests {
    use crate::dealer::Dealer;
    use crate::dealer::Deck;

    #[test]
    fn test_build_deck() {
        let deck = Deck::build();
        let expected_size = 52;

        assert_eq!(&expected_size, &deck.cards.len());
    }

    #[test]
    fn test_shuffle_deck() {
        // Test that shuffle works with no panics
        let mut dealer = Dealer::new();

        dealer.shuffle_deck();

        assert_eq!(&Deck::DECK_SIZE, &dealer.deck.cards.len());
    }

    #[test]
    fn test_create_dealer() {
        assert!(matches!(Some(Dealer::new()), Some(_)));
    }
}

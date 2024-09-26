use std::cmp;
use std::fmt::Debug;
use std::rc::Rc;

use rand::{Rng, thread_rng};

#[derive(Debug)]
enum Suit {
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
enum Rank {
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

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    const MAX_HAND_SIZE: usize = 10;
    fn new() -> Self {
        Self {
            cards: Vec::with_capacity(Hand::MAX_HAND_SIZE),
        }
    }

    fn get_score(&self) -> u32 {
        let mut ranks = Vec::with_capacity(Hand::MAX_HAND_SIZE);
        for card in &self.cards {
            ranks.push(resolve_rank(&card.rank))
        }
        ranks.iter().sum()
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

    pub fn hit<'a>(&'a mut self, hand: &'a mut Hand) -> &mut Hand {
        hand.cards.push(self.pop_card());
        println!("Hit! New hand: {:?}", hand.get_score());
        print_collection(&hand.cards);
        hand
    }

    pub fn deal_new_hand(&mut self) -> (Hand, Hand) {
        let mut dealer_hand = Hand::new();
        let mut player_hand = Hand::new();

        player_hand.cards.push(self.pop_card());
        dealer_hand.cards.push(self.pop_card());

        println!("Dealer's hand: {:?}", &dealer_hand.get_score());
        print_collection(&dealer_hand.cards);

        player_hand.cards.push(self.pop_card());
        dealer_hand.cards.push(self.pop_card());

        println!("Player's hand: {:?}", &dealer_hand.get_score());
        print_collection(&dealer_hand.cards);

        (dealer_hand, player_hand)
    }

    fn pop_card(&mut self) -> Card {
        self.deck.cards.pop().unwrap()
    }
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
    use std::rc::Rc;

    use crate::dealer::{Dealer, Rank};
    use crate::dealer::Deck;

    use super::*;

    #[test]
    fn test_build_deck() {
        let deck = Deck::build();
        assert_eq!(&Deck::DECK_SIZE, &deck.cards.len());
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

    #[test]
    fn test_resolve_rank() {
        let mut ranks = Vec::with_capacity(10);
        for rank in &vec![Rc::new(Rank::Jack), Rc::new(Rank::Ace)] {
            ranks.push(resolve_rank(rank));
        }

        assert_eq!(ranks, vec![10, 11])
    }
}

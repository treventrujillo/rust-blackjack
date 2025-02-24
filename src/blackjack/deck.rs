use std::rc::Rc;

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
    rank: Rc<Rank>,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub const DECK_SIZE: usize = 52;
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
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    const MAX_HAND_SIZE: usize = 10;
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(Hand::MAX_HAND_SIZE),
        }
    }

    pub fn get_score(&self) -> u32 {
        let mut ranks = Vec::with_capacity(Hand::MAX_HAND_SIZE);
        for card in &self.cards {
            ranks.push(crate::blackjack::dealer::resolve_rank(&card.rank))
        }
        ranks.iter().sum()
    }
}
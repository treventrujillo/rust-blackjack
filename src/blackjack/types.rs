use std::fmt;
use std::rc::Rc;

/// Represents the four suits in a standard deck
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

/// Represents the thirteen ranks in a standard deck
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    /// Returns the base value of the rank (Ace is 11, face cards are 10)
    pub fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    /// Returns true if this rank is an Ace
    pub fn is_ace(&self) -> bool {
        matches!(self, Rank::Ace)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

/// Represents a single playing card
#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Rc<Suit>,
    pub rank: Rc<Rank>,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit: Rc::new(suit),
            rank: Rc::new(rank),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

/// Represents possible player actions during their turn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Hit,
    Stand,
    DoubleDown,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Hit => write!(f, "Hit"),
            Action::Stand => write!(f, "Stand"),
            Action::DoubleDown => write!(f, "Double Down"),
        }
    }
}

/// Represents the outcome of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOutcome {
    PlayerWins,
    DealerWins,
    Push,
    InProgress,
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameOutcome::PlayerWins => write!(f, "Player wins!"),
            GameOutcome::DealerWins => write!(f, "Dealer wins!"),
            GameOutcome::Push => write!(f, "Push! It's a tie."),
            GameOutcome::InProgress => write!(f, "Game in progress"),
        }
    }
}

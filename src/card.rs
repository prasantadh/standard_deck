use std::fmt::Display;

use crate::Rank;
use crate::Suit;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: &Rank, suit: &Suit) -> Card {
        Card {
            rank: *rank,
            suit: *suit,
        }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.suit, self.rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

#[cfg(test)]
mod tests {
    use crate::Card;

    #[test]
    fn new_works() {
        let rank = crate::Rank::Two;
        let suit = crate::Suit::Clubs;
        assert!(
            Card::new(&rank, &suit)
                == Card {
                    rank: crate::Rank::Two,
                    suit: crate::Suit::Clubs
                }
        )
    }
}

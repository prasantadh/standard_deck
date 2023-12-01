use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn iter() -> SuitIterator {
        SuitIterator {
            index: Suit::Clubs as isize,
        }
    }
}

pub struct SuitIterator {
    index: isize, // to be initialized with Suit::Clubs only
}

impl Iterator for SuitIterator {
    type Item = Suit;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(Suit::Clubs),
            1 => Some(Suit::Diamonds),
            2 => Some(Suit::Hearts),
            3 => Some(Suit::Spades),
            _ => None,
        };
        self.index += 1;
        return result;
    }
}

impl Display for Suit {
    // this is for ease of local rendering
    // TODO: for json send out something different
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Clubs => "\u{2663}\u{FE0F}",
            Self::Diamonds => "\u{2666}\u{FE0F}",
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        };
        write!(f, "{}", c)
    }
}

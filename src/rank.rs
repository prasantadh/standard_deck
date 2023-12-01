use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Two = 2,
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
    pub fn iter() -> RankIterator {
        RankIterator {
            index: Rank::Two as isize,
        }
    }
}

pub struct RankIterator {
    index: isize, // to be initialized with Suit::Clubs only
}

impl Iterator for RankIterator {
    type Item = Rank;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            14 => Some(Rank::Ace),
            _ => None,
        };
        self.index += 1;
        return result;
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Jack => String::from("J"),
            Self::Queen => String::from("Q"),
            Self::King => String::from("K"),
            Self::Ace => String::from("A"),
            other => (*other as isize).to_string(),
        };
        write!(f, "{}", c)
    }
}

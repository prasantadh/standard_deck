use crate::{card::Card, rank::Rank, suit::Suit};

use rand::seq::SliceRandom;

pub struct Deck {
    cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = [Card::new(&Rank::Two, &Suit::Clubs); 52];
        let mut count = 0;
        for rank in Rank::iter() {
            for suit in Suit::iter() {
                deck[count] = Card::new(&rank, &suit);
                count += 1;
            }
        }
        deck.shuffle(&mut rand::thread_rng());
        return Deck { cards: deck };
    }

    pub fn get_cards(&self) -> &[Card; 52] {
        &self.cards
    }
}

// write a test to see if all the cards are returns by deck.new
#[cfg(test)]
mod tests {
    use crate::{card::Card, deck::Deck, rank::Rank, suit::Suit};

    #[test]
    fn deck_new_returns_standard_deck() {
        let mut count = 0;
        let deck = Deck::new();
        for rank in Rank::iter() {
            for suit in Suit::iter() {
                let card = &Card::new(&rank, &suit);
                for deck_card in deck.get_cards() {
                    if card == deck_card {
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, 52);
    }
}

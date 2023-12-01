use std_deck::Deck;

pub fn main() {
    let deck = Deck::new();
    for &card in deck.get_cards() {
        print!("{} ", card);
    }
    return;
}

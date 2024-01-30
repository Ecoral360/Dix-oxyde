use crate::types::{Card, Game, Rank, Suit};

/// Returns the bid that we should make.
pub fn get_bid_to_make(game: &Game) -> u8 {
    0
}

/// Returns the card that we should play.
pub fn get_next_card_to_play(game: &Game) -> Card {
    let playable_cards = game.playable_cards();

    playable_cards[0]
}

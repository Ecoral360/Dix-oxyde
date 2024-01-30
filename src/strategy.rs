use crate::game::{Card, Game, Rank, Suit};

/// Returns the bid that we should make.
pub fn make_bid(game: &Game) -> u8 {
    let hand = game.hand();

    let current_bid = game.winning_bid().unwrap_or_default();

    0
}

/// Returns the card that we should play.
pub fn choose_card(game: &Game) -> Card {
    let playable_cards = game.playable_cards();

    playable_cards[0]
}

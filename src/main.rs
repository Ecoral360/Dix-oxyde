use anyhow::Result;

use dix_oxyde::{
    game::{Card, Game},
    play_game,
    strategy::{BidStrategy, PlayStrategy},
};

#[derive(Default)]
struct MyStrategy;

impl BidStrategy for MyStrategy {
    fn make_bid(&mut self, game: &Game) -> u8 {
        let hand = game.hand();

        let current_bid = game.winning_bid().unwrap_or_default();

        let empty_suits = hand.empty_suits();

        // We have no empty suits. :(
        if empty_suits.is_empty() {}

        0
    }
}

impl PlayStrategy for MyStrategy {
    fn choose_card(&mut self, game: &Game) -> Card {
        let playable_cards = game.playable_cards();

        playable_cards[0]
    }
}

// Dix-oxyde
fn main() -> Result<()> {
    flexi_logger::Logger::try_with_str("info")?
        .format(flexi_logger::colored_default_format)
        .start()?;

    let mut strategy = MyStrategy::default();
    play_game(&mut strategy)?;
    Ok(())
}

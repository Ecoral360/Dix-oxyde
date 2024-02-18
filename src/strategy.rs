use crate::game::{Card, Game, Hand, Rank, Suit};

pub trait BidStrategy {
    fn setup_bid_phase(&mut self, game: &Game) {}

    fn make_bid(&mut self, game: &Game) -> u8;

    fn after_bid_phase(&mut self, game: &Game) {}
}

pub trait PlayStrategy {
    fn setup_trick(&mut self, game: &Game) {}

    fn choose_card(&mut self, game: &Game) -> Card;

    fn after_trick(&mut self, game: &Game) {}
}

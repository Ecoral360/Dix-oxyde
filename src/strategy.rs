use crate::game::{Card, Game, Hand, Rank, Suit};

pub trait BidStrategy {
    fn make_bid(&mut self, game: &Game) -> u8;
}

pub trait PlayStrategy {
    fn choose_card(&mut self, game: &Game) -> Card;
}

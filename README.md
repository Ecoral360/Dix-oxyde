# Dix-oxyde: The "Dix" bot written in Rust

### Instructions

To implement your own strategy, all you should need to do is complete the 
```rs
pub fn get_bid_to_make(game: &Game) -> u8 {
    ...
}

/// Returns the card that we should play.
pub fn get_next_card_to_play(game: &Game) -> Card {
    ...
}
```
in the `strategy.rs` file.

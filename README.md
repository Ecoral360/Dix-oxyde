# Dix-oxyde: The "Dix" bot written in Rust

### Instructions

To implement your own strategy, all you should need to do is complete the 
```rs
pub fn make_bid(game: &Game) -> u8 {
    ...
}

/// Returns the card that we should play.
pub fn choose_card(game: &Game) -> Card {
    ...
}
```
in the `strategy.rs` file.

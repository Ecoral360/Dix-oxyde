#![allow(unused_variables)]

use anyhow::Result;

use dix_oxyde::{
    game::{Card, Deck, Game},
    play_game,
    strategy::{BidStrategy, PlayStrategy},
};
use log::info;

#[derive(Default, Debug)]
struct MyStrategy {
    attacking: bool,
    other_players: [Deck; 3],
}

impl BidStrategy for MyStrategy {
    fn make_bid(&mut self, game: &Game) -> u8 {
        let hand = game.hand();

        let current_bid = game.winning_bid().unwrap_or_default();

        let empty_suits = hand.empty_suits();

        // We have no empty suits. :(
        if empty_suits.is_empty() {}

        0
    }

    fn after_bid_phase(&mut self, game: &Game) {
        // Safe to unwrap because we are after the bid phase
        let winner = game.bid_record().winner().unwrap();

        self.attacking = winner == *game.me() || winner == game.team_mate();

        let deck = game.deck_witout_hand();
        // let mut other_players = [Deck::new(); 3];
    }
}

impl PlayStrategy for MyStrategy {
    fn choose_card(&mut self, game: &Game) -> Card {
        let playable_cards = game.playable_cards();

        let master_cards = game.hand().master_cards(game.deck());
        if !master_cards.is_empty() {
            info!("Playing master card: {}", master_cards[0]);
            info!("deck: {:?}", game.deck());
            return master_cards[0];
        }

        playable_cards[0]
    }

    fn after_trick(&mut self, game: &Game) {
        todo!()
    }
}

// Dix-oxyde
fn main() -> Result<()> {
    // flexi_logger::Logger::try_with_str("info")?
    //     .format(flexi_logger::colored_default_format)
    //     .start()?;

    // let ppid = std::os::unix::process::parent_id();
    //
    // let command = std::process::Command::new("ps")
    //     .arg("-p")
    //     .arg(ppid.to_string())
    //     .arg("-o")
    //     .arg("pid,ppid,command")
    //     .output()
    //     .expect("failed to execute ps");

    // info!("{}", String::from_utf8_lossy(&command.stdout));

    let mut strategy = MyStrategy::default();
    play_game(&mut strategy)?;

    Ok(())
}

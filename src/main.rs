#![allow(dead_code)]

mod strategy;
mod game;

use anyhow::Result;
use log::info;
use std::io::{BufRead, StdinLock};
use game::{Card, Game, Hand, PlayerId};

use crate::{
    strategy::{make_bid, choose_card},
    game::NB_PLAYERS,
};

fn read_line(stdin: &mut StdinLock) -> Result<String> {
    let mut next_line = String::new();
    stdin.read_line(&mut next_line)?;
    Ok(next_line)
}

fn read_line_trimed(stdin: &mut StdinLock) -> Result<String> {
    let mut next_line = String::new();
    stdin.read_line(&mut next_line)?;
    Ok(next_line.trim().to_string())
}

fn read_line_stripped(stdin: &mut StdinLock, prefix: &str) -> Result<String> {
    read_line_trimed(stdin)?
        .strip_prefix(prefix)
        .ok_or_else(|| anyhow::anyhow!("Invalid input: {}", prefix))
        .map(|s| s.to_string())
}

fn get_me(stdin: &mut StdinLock) -> Result<PlayerId> {
    let me = read_line_stripped(stdin, "player ")?.parse::<usize>()?;
    Ok(me)
}

fn get_hand(line: String) -> Result<Hand> {
    let hand = {
        let hand = line
            .split_whitespace()
            .skip(1) // skip the "hand" prefix
            .map(|s| Card::try_from(s.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        hand
    };

    Ok(Hand::new(hand))
}

fn bid(stdin: &mut StdinLock, game: &mut Game) -> Result<()> {
    let mut pass = 0;
    let mut did_bid = [false; NB_PLAYERS];

    while pass < NB_PLAYERS - 1 || !did_bid.iter().all(|b| *b) {
        let next_line = read_line_stripped(stdin, "bid ")?;

        if next_line == "?" {
            println!("{}", make_bid(&game));
            continue;
        }

        let (player, bid) = next_line.split_once(' ').unwrap();
        let player = player.parse::<usize>()?;
        let bid = bid.parse::<u8>()?;
        game.player_bid(player, bid);

        // Player passed
        if bid == 0 {
            pass += 1;
        }
        did_bid[player] = true;
    }
    info!("Winning bid: {}", game.winning_bid().unwrap_or(0));
    Ok(())
}

fn play_trick(stdin: &mut StdinLock, game: &mut Game) -> Result<()> {
    let mut played = 0;

    while played < NB_PLAYERS {
        let next_line = read_line_stripped(stdin, "card ")?;

        if next_line == "?" {
            let card = choose_card(&game);
            game.play_card(card);
            println!("{}", card);
            continue;
        }

        let (player, bid) = next_line.split_once(' ').unwrap();
        let player = player.parse::<usize>()?;
        let card = Card::try_from(bid.to_string())?;
        game.card_played(player, card);

        played += 1;
    }

    Ok(())
}

fn play_round(stdin: &mut StdinLock, me: PlayerId) -> Result<bool> {
    let first_line = read_line_trimed(stdin)?;
    if first_line == "end" {
        return Ok(false);
    }

    let hand = get_hand(first_line)?;

    let nb_tricks = hand.cards().len();

    let mut game = Game::new(me, hand);

    bid(stdin, &mut game)?;

    for _ in 0..nb_tricks {
        play_trick(stdin, &mut game)?;
    }

    Ok(true)
}

// Dix-oxyde
fn main() -> Result<()> {
    flexi_logger::Logger::try_with_str("info")?
        .format(flexi_logger::colored_default_format)
        .start()?;

    let mut stdin = std::io::stdin().lock();

    let me = get_me(&mut stdin)?;

    while play_round(&mut stdin, me)? {
    }

    Ok(())
}

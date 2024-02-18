#![allow(dead_code)]

pub mod game;
pub mod strategy;
pub mod startegies;

use anyhow::Result;
use game::{Card, Game, Hand, PlayerId, NB_PLAYERS};
use log::info;
use std::io::{BufRead, StdinLock};
use strategy::{BidStrategy, PlayStrategy};

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

pub fn get_me(stdin: &mut StdinLock) -> Result<PlayerId> {
    let me = read_line_stripped(stdin, "player ")?.parse::<usize>()?;
    Ok(me)
}

pub fn get_hand(line: String) -> Result<Hand> {
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

pub fn bid<S>(stdin: &mut StdinLock, game: &mut Game, strategy: &mut S) -> Result<()>
where
    S: BidStrategy,
{
    let mut pass = 0;
    let mut did_bid = [false; NB_PLAYERS];

    while pass < NB_PLAYERS - 1 || !did_bid.iter().all(|b| *b) {
        let next_line = read_line_stripped(stdin, "bid ")?;

        if next_line == "?" {
            println!("{}", strategy.make_bid(&game));
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

fn play_trick<S>(stdin: &mut StdinLock, game: &mut Game, strategy: &mut S) -> Result<()>
where
    S: PlayStrategy,
{
    let mut played = 0;

    while played < NB_PLAYERS {
        let next_line = read_line_stripped(stdin, "card ")?;

        if next_line == "?" {
            let card = strategy.choose_card(&game);
            println!("{}", card);
            continue;
        }

        let (player, card) = next_line.split_once(' ').unwrap();
        let player = player.parse::<usize>()?;
        let card = Card::try_from(card.to_string())?;
        game.card_played(player, card);

        played += 1;
    }

    Ok(())
}

pub fn play_round<S>(stdin: &mut StdinLock, me: PlayerId, strategy: &mut S) -> Result<bool>
where
    S: BidStrategy + PlayStrategy,
{
    let first_line = read_line_trimed(stdin)?;
    if first_line == "end" {
        return Ok(false);
    }

    let hand = get_hand(first_line)?;

    let nb_tricks = hand.cards().len();

    let mut game = Game::new(me, hand);

    strategy.setup_bid_phase(&game);

    bid(stdin, &mut game, strategy)?;

    strategy.after_bid_phase(&game);

    for _ in 0..nb_tricks {
        strategy.setup_trick(&game);
        play_trick(stdin, &mut game, strategy)?;
        strategy.after_trick(&game);
    }

    Ok(true)
}

pub fn play_game<S>(strategy: &mut S) -> Result<()>
where
    S: BidStrategy + PlayStrategy,
{
    let mut stdin = std::io::stdin().lock();

    let me = get_me(&mut stdin)?;

    while play_round(&mut stdin, me, strategy)? {}

    Ok(())
}

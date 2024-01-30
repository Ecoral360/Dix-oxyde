use anyhow::anyhow;
use derive_getters::Getters;
use derive_new::new;
use std::fmt::Display;

pub const NB_SUITS: usize = 4;
pub const NB_PLAYERS: usize = 4;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn order(&self) -> usize {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_string = match self {
            Suit::Spades => "S",
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
        };

        write!(f, "{}", to_string)
    }
}

impl TryFrom<char> for Suit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(match value {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            _ => Err(anyhow!("Invalid suit"))?,
        })
    }
}

pub const NB_RANKS: usize = 10;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
}

impl Rank {
    pub fn order(&self) -> usize {
        match self {
            Rank::Five => 0,
            Rank::Six => 1,
            Rank::Seven => 2,
            Rank::Eight => 3,
            Rank::Nine => 4,
            Rank::Ten => 5,
            Rank::Jack => 6,
            Rank::Queen => 7,
            Rank::King => 8,
            Rank::Ace => 9,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_string = match self {
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Ten => "T",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
        };

        write!(f, "{}", to_string)
    }
}

impl TryFrom<char> for Rank {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(match value {
            'A' => Rank::Ace,
            'K' => Rank::King,
            'Q' => Rank::Queen,
            'J' => Rank::Jack,
            'T' => Rank::Ten,
            '9' => Rank::Nine,
            '8' => Rank::Eight,
            '7' => Rank::Seven,
            '6' => Rank::Six,
            '5' => Rank::Five,
            _ => Err(anyhow!("Invalid rank"))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Getters, new)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl TryFrom<String> for Card {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut chars = value.chars();
        let suit = Suit::try_from(chars.next().ok_or(anyhow!("Invalid End Of String"))?)?;
        let rank = Rank::try_from(chars.next().ok_or(anyhow!("Invalid End Of String"))?)?;
        Ok(Card { suit, rank })
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

pub type PlayerId = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn playable_cards(&self, ongoing_trick: &OngoingTrick) -> Vec<Card> {
        if ongoing_trick.is_empty() {
            return self.cards.iter().copied().collect();
        }

        // It's safe to unwrap here because we know that the trick is not empty.
        let follow = ongoing_trick.follow().unwrap();

        let followed = self
            .cards
            .iter()
            .filter(|c| c.suit() == &follow)
            .copied()
            .collect::<Vec<Card>>();

        if followed.is_empty() {
            self.cards.iter().copied().collect()
        } else {
            followed
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters, Default)]
pub struct OngoingTrick {
    cards: Vec<(PlayerId, Card)>,
    #[new(default)]
    follow: Option<Suit>,
    trump: Option<Suit>,
}

impl OngoingTrick {
    pub fn push(&mut self, player: PlayerId, card: Card) {
        if self.trump.is_none() {
            self.trump = Some(*card.suit());
        }

        if self.follow.is_none() {
            self.follow = Some(*card.suit());
        }

        self.cards.push((player, card));
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() == NB_PLAYERS
    }

    pub fn clear(&mut self) {
        self.cards.clear();
        self.follow = None;
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the current trick winner.
    pub fn get_current_trick_winner(&self) -> Option<PlayerId> {
        // If the trick is empty, we are the first to play.
        if self.is_empty() {
            return None;
        }

        let follow = self.follow().unwrap();
        let trump = self.trump().unwrap();

        Some(
            self.cards
                .iter()
                .max_by_key(|(_, c)| {
                    let order = match c.suit() {
                        s if s == &trump => 5,
                        s if s == &follow => 4,
                        s => s.order(),
                    };

                    NB_RANKS * order + c.rank().order()
                })
                .unwrap()
                .0,
        )
    }
}

impl Into<Trick> for &OngoingTrick {
    fn into(self) -> Trick {
        Trick {
            cards: self.cards.clone().try_into().unwrap(),
            winner: self.get_current_trick_winner().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters)]
pub struct Trick {
    cards: [(PlayerId, Card); NB_PLAYERS],
    winner: PlayerId,
}

impl Trick {
    pub fn get_nb_points(&self) -> usize {
        self.cards
            .iter()
            .filter_map(|(_, c)| match c.rank() {
                Rank::Ace => Some(10),
                Rank::Ten => Some(10),
                Rank::Five => Some(5),
                _ => None,
            })
            .sum::<usize>()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters, Default)]
pub struct Round {
    history: Vec<Trick>,
    ongoing: OngoingTrick,
    #[new(default)]
    trump: Option<Suit>,
}

impl Round {
    pub fn play_card(&mut self, player: PlayerId, card: Card) {
        self.ongoing.push(player, card);

        if self.ongoing.is_full() {
            // it's safe to unwrap here because we know that the trick is full.
            self.history.push(self.ongoing().into());
            self.ongoing.clear();
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters, Default)]
pub struct BidRecord {
    bids: [u8; NB_PLAYERS],
    winner: Option<PlayerId>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, new, Getters)]
pub struct Game {
    me: PlayerId,
    hand: Hand,
    #[new(default)]
    bid_record: BidRecord,
    #[new(default)]
    round: Round,
}

/// Impl block with accessors.
impl Game {
    pub fn trump(&self) -> Option<Suit> {
        *self.round.trump()
    }

    pub fn team_mate(&self) -> PlayerId {
        (self.me + 2) % NB_PLAYERS
    }

    pub fn winning_bid(&self) -> Option<u8> {
        self.bid_record.winner.map(|p| self.bid_record.bids[p])
    }

    pub fn get_current_trick_winner(&self) -> Option<PlayerId> {
        self.round.ongoing().get_current_trick_winner()
    }

    pub fn get_current_trick(&self) -> &OngoingTrick {
        self.round.ongoing()
    }

    pub fn playable_cards(&self) -> Vec<Card> {
        self.hand.playable_cards(self.round.ongoing())
    }
}

impl Game {
    pub fn player_bid(&mut self, player: PlayerId, bid: u8) {
        if bid != 0 {
            self.bid_record.winner = Some(player);
            self.bid_record.bids[player] = bid;
        }
    }

    pub fn play_card(&mut self, card: Card) {
        self.hand.cards.retain(|c| c != &card);
    }

    pub fn card_played(&mut self, player: PlayerId, card: Card) {
        if self.trump().is_none() {
            self.round.trump = Some(*card.suit());
        }

        self.round.play_card(player, card);
    }
}

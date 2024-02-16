#![allow(dead_code)]
use std::{cmp::Ordering, fmt::Display};

use rand::{seq::SliceRandom, thread_rng};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u32),
    Jack,
    Queen,
    King,
}
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Rank::Ace => match other {
                Rank::Ace => Ordering::Equal,
                _ => Ordering::Less,
            },
            Rank::Number(rank) => match other {
                Rank::Ace => Ordering::Greater,
                Rank::Number(other_rank) => rank.cmp(other_rank),
                _ => Ordering::Less,
            },
            Rank::Jack => match other {
                Rank::Ace | Rank::Number(_) => Ordering::Greater,
                Rank::Jack => Ordering::Equal,
                _ => Ordering::Less
            },
            Rank::Queen => match other {
                Rank::Ace | Rank::Number(_) | Rank::Jack => Ordering::Greater,
                Rank::Queen => Ordering::Equal,
                Rank::King => Ordering::Less,
            },
            Rank::King => match other {
                Rank::King => Ordering::Equal,
                _ => Ordering::Greater
            },
        }
    }
}
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Rank {
    fn get_unicode_offset(&self) -> u32 {
        match self {
            Rank::Ace => 1,
            Rank::Number(val) => *val,
            Rank::Jack => 11,
            Rank::Queen => 13,
            Rank::King => 14,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suits {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Suits {
    fn get_unicode_offset(&self) -> u32 {
        match self {
            Suits::Spades => 0x00,
            Suits::Hearts => 0x10,
            Suits::Diamonds => 0x20,
            Suits::Clubs => 0x30,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CardFace {
    pub rank: Rank,
    pub suit: Suits,
}
impl Ord for CardFace {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.suit.cmp(&other.suit) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.rank.cmp(&other.rank) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Greater => Ordering::Greater,
        }
    }
}
impl PartialOrd for CardFace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Display for CardFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_unicode = 0x1F0A0 + self.suit.get_unicode_offset() + self.rank.get_unicode_offset();
                let card_char = char::from_u32(card_unicode).unwrap();
                write!(f, "{card_char} ")
    }
}
#[derive(Clone, Copy, Default)]
pub struct Card {
    pub card: Option<CardFace>,
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.card {
            Some(card) => {
                let card_unicode = 0x1F0A0 + card.suit.get_unicode_offset() + card.rank.get_unicode_offset();
                let card_char = char::from_u32(card_unicode).unwrap();
                write!(f, "{card_char} ")
            }
            None => write!(f, "\u{1F0A0} "),
        }
    }
}
pub struct Deck {
    cards: Vec<CardFace>,
    taken_cards: Vec<CardFace>
}
impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }
    pub fn sort(&mut self) {
        self.cards.sort()
    }
    pub fn draw_card(&mut self) -> Option<CardFace> {
        let card = self.cards.pop();
        if let Some(card) = card {
            self.taken_cards.push(card);
        }
        card
    }
    pub fn return_card(&mut self, card: CardFace) -> bool{
        self.taken_cards.sort();
        if let Ok(index) = self.taken_cards.binary_search(&card) {
            self.taken_cards.remove(index);
            self.cards.insert(0, card);
            true
        } else {
            false
        }
    }
    pub fn return_all_cards(&mut self) {
        self.cards.append(&mut self.taken_cards);
    }
    pub fn from_cards(cards: Vec<CardFace>) -> Self {
        Self {
            cards,
            taken_cards: Vec::new()
        }
    }
    pub fn cards(&self) -> Vec<CardFace> {
        self.cards.clone()
    }
}
impl Default for Deck {
    fn default() -> Self {
        Self { cards: vec![
            CardFace{suit: Suits::Hearts, rank: Rank::King},
            CardFace{suit: Suits::Hearts, rank: Rank::Queen},
            CardFace{suit: Suits::Hearts, rank: Rank::Jack},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(10)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(9)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(8)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(7)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(6)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(5)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(4)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(3)},
            CardFace{suit: Suits::Hearts, rank: Rank::Number(2)},
            CardFace{suit: Suits::Hearts, rank: Rank::Ace},
            
            CardFace{suit: Suits::Clubs, rank: Rank::King},
            CardFace{suit: Suits::Clubs, rank: Rank::Queen},
            CardFace{suit: Suits::Clubs, rank: Rank::Jack},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(10)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(9)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(8)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(7)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(6)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(5)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(4)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(3)},
            CardFace{suit: Suits::Clubs, rank: Rank::Number(2)},
            CardFace{suit: Suits::Clubs, rank: Rank::Ace},
            
            CardFace{suit: Suits::Diamonds, rank: Rank::King},
            CardFace{suit: Suits::Diamonds, rank: Rank::Queen},
            CardFace{suit: Suits::Diamonds, rank: Rank::Jack},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(10)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(9)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(8)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(7)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(6)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(5)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(4)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(3)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Number(2)},
            CardFace{suit: Suits::Diamonds, rank: Rank::Ace},
            
            CardFace{suit: Suits::Spades, rank: Rank::King},
            CardFace{suit: Suits::Spades, rank: Rank::Queen},
            CardFace{suit: Suits::Spades, rank: Rank::Jack},
            CardFace{suit: Suits::Spades, rank: Rank::Number(10)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(9)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(8)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(7)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(6)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(5)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(4)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(3)},
            CardFace{suit: Suits::Spades, rank: Rank::Number(2)},
            CardFace{suit: Suits::Spades, rank: Rank::Ace},
            ], taken_cards: Vec::new() }
    }
}
impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cards in deck:\n")?;
        for card in self.cards.iter() {
            write!(f, "{} ", card)?;
        }
        write!(f, "\nCards taken from deck:\n")?;
        for card in self.taken_cards.iter() {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}
pub trait HandValue {
    fn value(&self) -> u32 {
        todo!()
    }
}
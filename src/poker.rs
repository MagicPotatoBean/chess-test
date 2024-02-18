use std::{fmt::Display, io::Write};

use crate::{cards::{self, CardFace, Deck}, readline};
pub fn main() {
    let mut state = GameState::default();
    let mut community_cards = Hand::default();
    let mut opponent_cards: Vec<Hand> = Vec::new();
    let mut player_cards = Hand::default();
    let mut deck = Deck::default();
    let mut player_bet = 0;
    let mut opponent_bet = 0;
    let mut player_money = 100;
    let mut folded = false;
    'outer_loop: loop {
        match state {
            GameState::Setup => {
                folded = false;
                player_bet = 1;
                opponent_bet = 1;
                deck = Deck::default();
                deck.shuffle();
                community_cards.cards.clear();
                player_cards.cards.clear();
                for _ in 0..5 {
                    community_cards.cards.push(deck.draw_card().unwrap());
                }
                for _ in 0..3 {
                    let mut hand = Hand::default();
                    hand.cards.push(deck.draw_card().unwrap());
                    hand.cards.push(deck.draw_card().unwrap());
                    opponent_cards.push(hand);
                }
                player_cards.cards.push(deck.draw_card().unwrap());
                player_cards.cards.push(deck.draw_card().unwrap());
                state = GameState::NoCardsShown;
            }
            GameState::NoCardsShown => {
                'inner_loop: loop {
                    println!("Community cards: ");
                    println!("{0}{0}{0}{0}{0}", cards::Card::default());
                    println!("Your cards: ");
                    println!("{player_cards}");
                    if let Some(user_input) = readline!() {
                        let input = user_input.trim().to_lowercase();
                        match input.as_str() {
                            "menu" => {if confirm() {
                                break 'outer_loop
                            }},
                            "fold" => {
                                player_money -= player_bet;
                                folded = true;
                                state = GameState::AllCardsShown;
                                break 'inner_loop;
                            },
                            "check" => {
                                player_bet = opponent_bet;
                                state = GameState::OneCardShown;
                                break 'inner_loop;
                            },
                            other => {
                                if let Ok(amount) = prse::try_parse!(other, "raise {}").map(|val: i32| val) {
                                    player_bet = opponent_bet + amount;
                                    state = GameState::OneCardShown;
                                break 'inner_loop;
                            }
                            },
                        };
                    }
                }
            },
            GameState::OneCardShown => {

                println!("Community cards: ");
                println!("{1}{0}{0}{0}{0}", cards::Card::default(), community_cards.cards[0]);
                println!("Your cards: ");
                println!("{player_cards}");
            },
            GameState::TwoCardsShown => todo!(),
            GameState::AllCardsShown => {
                println!("Community cards: ");
                    println!("{}{}{}{}{}", community_cards.cards[0], community_cards.cards[1], community_cards.cards[2], community_cards.cards[3], community_cards.cards[4]);
                    println!("Your cards: ");
                    println!("{player_cards}");
                    println!("Opponent cards: ");
                    for (index, hand) in opponent_cards.iter().enumerate() {
                        if index == 0 {
                            print!("{hand}");
                        } else {
                            print!(", {hand}");
                        }
                    }
                    println!();
                state = GameState::Setup;
            },
        }
    }
}
fn evaluate_winner(community_cards: &Hand, _player_hand: &Hand, opponent_hands: &[Hand]) -> Players {
    {
        let mut max_score = 0;
        let mut winner: Option<u32> = None;
        for (index, hand) in opponent_hands.iter().enumerate() {
            let hand_score = evaluate_hand(community_cards, hand);  
            if hand_score > max_score {
                if let Ok(opponent) = index.try_into() {
                    max_score = hand_score;
                    winner = Some(opponent);
                }
            }
        }
        

    };
    todo!()
}
fn evaluate_hand(_community_cards: &Hand, _hand: &Hand) -> u32 {  
    todo!()
}
enum Players {
    Player,
    Opponent(u32),
    Draw,
}
fn confirm() -> bool {
    print!("Are you sure? [y/n]: ");
    let _ = std::io::stdout().flush();
    let mut line = String::default();
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().eq_ignore_ascii_case("y")
}
#[derive(Default)]
enum GameState {
    #[default]
    Setup,
    NoCardsShown,
    OneCardShown,
    TwoCardsShown,
    AllCardsShown,
}
#[derive(Default)]
struct Hand {
    cards: Vec<CardFace>,
}
// impl HandValue for Hand {
// 
// }
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{card}")?;
        }
        Ok(())
    }
}
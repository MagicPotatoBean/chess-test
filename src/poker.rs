use std::{default, fmt::Display, io::Write};

use crate::{cards::{self, *}, readline};
pub fn main() {
    let mut state = GameState::default();
    let mut community_cards = Hand::default();
    let mut opponent_cards: Vec<Hand> = Vec::new();
    let mut player_cards = Hand::default();
    loop {
        match state {
            GameState::NoCardsShown => {
                println!("Community cards: ");
                println!("{0}{0}{0}{0}{0}", cards::Card::default());
                println!("Your cards: ");
                println!("{}", player_cards);
                if let Some(user_input) = readline!() {
                    let input = user_input.trim().to_lowercase();
                    match input.as_str() {
                        "menu" => {if confirm() {
                            break
                        }},
                        "fold" => {},
                        other => {},
                    };
                }
            },
            GameState::OneCardShown => {

                println!("Community cards: ");
                println!("{1}{0}{0}{0}{0}", cards::Card::default(), community_cards.cards[0]);
                println!("Your cards: ");
                println!("{}", player_cards);
            },
            GameState::TwoCardsShown => todo!(),
            GameState::AllCardsShown => todo!(),
        }
    }
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
        for card in self.cards.iter() {
            write!(f, "{}", card)?
        }
        Ok(())
    }
}
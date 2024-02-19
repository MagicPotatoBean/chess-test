use crate::{cards::{CardFace, Deck, HandValue, Rank}, readline};
use colored::Colorize;
use std::{fmt::Display, io::Write};
pub fn main() {
    let mut deck = Deck::default();
    deck.shuffle();
    let mut dealer_hand: Hand;
    let mut player_hand: Hand = Hand::default();
    let mut player_wins: u32 = 0;
    let mut dealer_wins: u32 = 0;
    help_menu();
    'outer_loop: loop {
        println!("----- Blackjack -----");
        println!("Wins:Losses = {player_wins}:{dealer_wins}");
        deck = Deck::default();
        deck.shuffle();
        dealer_hand = {
            if let Some(hand) = play_dealer(&mut deck) {
                hand
            } else {
                println!("Encountered a fatal error, returning to menu.");
                return
            }
        };
        
        player_hand.cards.clear();
        player_hand.cards.push({
            if let Some(card) = deck.draw_card() {
                card
            } else {
                println!("Encountered a fatal error, returning to menu.");
                return
            }
        });
        player_hand.cards.push({
            if let Some(card) = deck.draw_card() {
                card
            } else {
                println!("Encountered a fatal error, returning to menu.");
                return
            }
        });
        'game_loop: loop {
            println!("Your cards: {} = {}", player_hand, player_hand.value());
            if player_hand.value() > 21 {
                print_winner(&player_hand, &dealer_hand, &mut player_wins, &mut dealer_wins);
                        print!("Would you like to play again? [y/n]:");
                        let _ = std::io::stdout().flush();
                if let Some(response) = readline!() {
                    if response.trim().to_lowercase().as_str() == "n" {
                        break 'outer_loop
                    }
                    break 'game_loop
                }
            }
            if let Some(user_input) = readline!() {
                let user_input = user_input.to_lowercase().trim().to_owned();
                if user_input == "menu" && confirm() {
                    break 'outer_loop;
                } else if user_input == "help" {
                    help_menu();
                } else if user_input == "reset" && confirm() {
                    break 'game_loop;
                } else if user_input == "hit" {
                    player_hand.cards.push({
                        if let Some(card) = deck.draw_card() {
                            card
                        } else {
                            println!("Encountered a fatal error, returning to menu.");
                            return
                        }
                    });
                } else if user_input == "stand" {
                        print_winner(&player_hand, &dealer_hand, &mut player_wins, &mut dealer_wins);
                        print!("Would you like to play again? [y/n]:");
                        let _ = std::io::stdout().flush();
                if let Some(response) = readline!() {
                    if response.trim().to_lowercase().as_str() == "n" {
                        break 'outer_loop
                    }
                    break 'game_loop
                }
                }
            }
        }
    }
}
fn print_winner(player_hand: &Hand, dealer_hand: &Hand, player_wins: &mut u32, dealer_wins: &mut u32) {
    println!("Dealer: {} = {}", dealer_hand, dealer_hand.value());
    println!();
    if player_hand.value() > 21 && dealer_hand.value() <= 21 {
        println!("You went bust, dealer wins.");
        *dealer_wins += 1;
    } else if dealer_hand.value() > 21 && player_hand.value() <= 21 {
        println!("Dealer went bust, you win.");
        *player_wins += 1;
    } else if player_hand.value() > 21 && dealer_hand.value() > 21 {
        println!("You and the dealer went bust, dealer wins.");
        *dealer_wins += 1;
    } else if player_hand.value() > dealer_hand.value() {
        println!("You win.");
        *player_wins += 1;
    } else {    
        println!("Dealer wins.");
        *dealer_wins += 1;
    }
}
fn help_menu() {
    println!();
    println!("Type \"{}\" to return to the menu.", "menu".red());
    // println!("Type \"{}\" to get the history of the game.", "history".red());
    println!("Type \"{}\" to show this menu.", "help".red());
    println!("Type \"{}\" to restart the game.", "reset".red());
    println!("Type \"{}\" to get another card.", "hit".red());
    println!("Type \"{}\" to keep this hand.", "stand".red());
    // println!("Type \"{}\" to save the state of the board.", "save".red());
    // println!("Type \"{}\" to load the state of the board.", "load".red());
    println!();
}
fn confirm() -> bool {
    print!("Are you sure? [y/n]: ");
    let _ = std::io::stdout().flush();
    let mut line = String::default();
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().eq_ignore_ascii_case("y")
}
#[derive(Default)]
struct Hand {
    cards: Vec<CardFace>,
}
impl HandValue for Hand {
    fn value(&self) -> u32 {
        let mut sum = 0;
        let mut ace_count = 0;
        for card in &self.cards {
            if let Rank::Ace = card.rank {
                ace_count += 1;
            } else {
                sum += card_value(card, false);
            }
        }
        for ace_low_count in 0..=ace_count {
            if sum + ace_low_count + ((ace_count - ace_low_count) * 11) <= 21 {
                return sum + ace_low_count + ((ace_count - ace_low_count) * 11);
            }
        }
        sum + ace_count
    }
}

fn card_value(card: &CardFace, ace_high: bool) -> u32 {
    match card.rank {
        Rank::Ace => {
            if ace_high {
                11
            } else {
                1
            }
        }
        Rank::Number(num) => num,
        Rank::Jack | Rank::Queen | Rank::King => 10,
    }
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{card}")?;
        }
        Ok(())
    }
}
fn play_dealer(deck: &mut Deck) -> Option<Hand> {
    let mut hand = Hand::default();
    loop {
        hand.cards.push(deck.draw_card()?);
        if hand.value() >= 17 {
            break;
        }
    }
    Some(hand)
}
#[cfg(test)]
mod tests {
    use crate::cards::Suits;

    #[test]
    fn hand_value_test() {
        use super::*;

        let mut hand = Hand::default();
        hand.cards.push(CardFace{rank: Rank::Ace, suit: Suits::Diamonds});
        hand.cards.push(CardFace{rank: Rank::Number(3), suit: Suits::Diamonds});
        hand.cards.push(CardFace{rank: Rank::Ace, suit: Suits::Spades});
        assert_eq!(hand.value(), 15);
    }
}
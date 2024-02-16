mod checkers;
mod chess;
// mod ludo;
mod backgammon;
mod blackjack;
mod cards;

fn main() {
    loop {
        println!(" Rust board games ");
        println!("------------------");
        println!("[C]hess");
        println!("C[h]eckers");
        // println!("[L]udo");
        println!("[B]ackgammon (WIP)");
        println!("B[l]ackjack");
        println!("[E]xit");
        let user_input = readline!();
        match user_input.map(|input| input.trim().to_lowercase()).as_deref() {
            Some("c") => chess::main(),
            Some("h") => checkers::main(),
            // Some("l") => ludo::main(),
            Some("b") => backgammon::main(),
            Some("l") => blackjack::main(),
            Some("e") => break,
            _ => {}
        }
        println!();
        println!();
        println!();
    }
}

#[macro_export]
macro_rules! readline {
    ($line: ident) => {{
        let result = std::io::stdin().read_line(&mut line);
        if result.ok().is_some() {
            Some(line)
        } else {
            None
        }
    }};
    () => {{
        let mut line: String = String::default();
        let result = std::io::stdin().read_line(&mut line);
        if result.ok().is_some() {
            Some(line)
        } else {
            None
        }
    }};
}
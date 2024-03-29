mod checkers;
mod chess;
// mod ludo;
mod backgammon;
mod blackjack;
mod cards;
mod poker;
mod dice;
fn main() {
    loop {
        println!(" Rust board games ");
        println!("------------------");
        // println!("4 Player games:");
        // println!("[L]udo");
        println!("2 Player games:");
        println!("    \u{2659} [C]hess");
        println!("    \u{026c1} C[h]eckers");
        println!("    \u{2684} [B]ackgammon");
        println!("1 Player games:");
        println!("    \u{1F0DB} B[l]ackjack");
        println!("    \u{2666} [P]oker (WIP)");
        println!();
        println!("[E]xit");
        let user_input = readline!();
        match user_input.map(|input| input.trim().to_lowercase()).as_deref() {
            Some("c") => chess::main(),
            Some("h") => checkers::main(),
            // Some("l") => ludo::main(),
            Some("b") => backgammon::main(),
            Some("l") => blackjack::main(),
            Some("p") => poker::main(),
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
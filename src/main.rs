use shakmaty::{Chess, Position};

use crate::engine::Engine;

mod engine;
mod eval;
mod search;

fn main() {
    let mut engine = Engine::new();
    let mut pos = Chess::new();

    let legals = pos.legal_moves();

    println!("============================================");
    println!("Chess Bot Started!");
    println!("============================================");

    println!("Legal moves num: {}", legals.len());

    loop {
        if pos.is_game_over()  {
            println!("Partie beendet.");
            break;
        }

        let Some(mv) = engine.find_best_move(&pos) else {
            break;
        };

        println!("Bot spielt {}", mv);

        pos = pos.play(mv).expect("Fehler denke ich")
    }
}
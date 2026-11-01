use std::io::{self, BufRead, BufWriter, Write};

use shakmaty::{CastlingMode, Chess, Position, fen::Fen};
use shakmaty_uci::{UciMessage, UciMove};

use crate::engine::Engine;

mod engine;
mod eval;
mod search;

fn main() -> io::Result<()> {
    eprintln!("============================================");
    eprintln!("Chess Bot Started!");
    eprintln!("============================================");
    
    // UCI Enginges geben Behle über stdin
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut out = BufWriter::new(stdout.lock());

    let mut engine = Engine::new();
    let mut pos = Chess::new();
    
    for line in stdin.lock().lines() {
        let line = line?;

        let message = match line.parse() {
            Ok(message) => message,
            Err(_) => {
                eprintln!("Invalid UCI command");
                continue;
            }
        };

        match message {
            // Handshake am Anfang
            UciMessage::Uci => {
                writeln!(out, "id name trinity")?;
                writeln!(out, "uciok")?;
            }
            UciMessage::IsReady => {
                writeln!(out, "readyok")?;
            }
            // Neue Position an die Engine übergeben
            UciMessage::Position { startpos, fen, moves } => {
                match set_position(startpos, fen, moves) {
                    Ok(new_pos) => {
                        pos = new_pos;
                    }
                    Err(err) => {
                        eprintln!("Error with position")
                    }
                }
            }
            // Bereit für den nächsten Zug
            // Time control etc muss noch gemacht werden
            UciMessage::Go { time_control, search_control } => {
                if pos.is_game_over()  {
                    writeln!(out, "bestmove 0000")?;
                } else {
                    let Some(mv) = engine.find_best_move(&pos) else {
                        writeln!(out, "bestmove 0000")?; // Nichts gefunden
                        eprintln!("No best move found!");
                        out.flush();
                        continue;
                    };

                    let uci_move = UciMove::from_standard(mv);

                    writeln!(out, "bestmove {uci_move}")?;
                }
            }
            // Neues Spiel starten
            UciMessage::UciNewGame => {
                engine = Engine::new();
                pos = Chess::default();
            }
            UciMessage::Quit => {
                break;
            }
            _ => {}
        }
        out.flush()?;
    }
    Ok(())
}


fn set_position(
    startpos: bool,
    fen: Option<Fen>,
    moves: Vec<UciMove>,
) -> Result<Chess, String> {

    let mut pos = if startpos {
        Chess::default()
    } else {
        fen.ok_or("Missing FEN")?
            .into_position(CastlingMode::Standard)
            .map_err(|err| format!("Invalid FEN: {err}"))?
    };

    // Apply all moves supplied by the GUI
    for uci_move in moves {
        let mv = uci_move
            .to_move(&pos)
            .map_err(|_| format!("Illegal move: {uci_move}"))?;

        pos = pos
            .play(mv)
            .map_err(|_| format!("Could not play move: {uci_move}"))?;
    }

    Ok(pos)
}
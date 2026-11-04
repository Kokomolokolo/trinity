use std::thread::panicking;

use shakmaty::{Chess, Move, Piece, Position, Role};

use crate::eval::{evaluate, piece_value};

const INF: i32 = 1_000_000;

pub fn search(
    pos: &Chess,
    depth: u8,
) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut alpha = -INF;
    let beta = INF;

    let moves = order_moves(
        &pos.legal_moves().to_vec(),
        pos
    );

    for m in moves {
        if let Ok(new_pos) = pos.clone().play(m) {
            let score = -alpha_beta(&new_pos, -beta, -alpha, depth);
            
            if score > alpha {
                alpha = score;
                best_move = Some(m);
            }
        }
    }
    best_move
}

fn alpha_beta(pos: &Chess, mut alpha: i32, mut beta: i32, depth: u8) -> i32 {
    let moves = order_moves( // Sortieren für besseren Alpha Beta cutoff
        &pos.legal_moves().to_vec(),
        pos
    );

    if moves.is_empty() {
        if pos.is_check() {
            return -100_000; // Schachmatt
        } else {
            return 0; // Patt
        }
    }

    if depth == 0 {
        return evaluate(pos); // Quisesecnte search hier noch einbauen
    }   

    let mut max = -INF;
    for m in moves {
        if let Ok(new_pos) = pos.clone().play(m) {
            let score = -alpha_beta(&new_pos, -beta, -alpha, depth - 1);

            max = max.max(score);
            
            alpha = alpha.max(score);
            
            if alpha >= beta {
                break;
            }
        }
    }
    max
}

fn order_moves(moves: &Vec<Move>, pos: &Chess) -> Vec<Move> {
    let mut scored_moves: Vec<(Move, i32)> = moves
        .iter()
        .map(|&mv| {
            let score = score_move(&mv, pos);
            (mv, score)
        })
        .collect();

    // Sortieren
    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    // Die Moves aus dem sortierten Tupel holen
    scored_moves.into_iter().map(|(mv, _)| mv ).collect()
}

fn score_move(mv: &Move, pos: &Chess) -> i32 {
    let mut score = 0; 
    if mv.is_capture() {
        let origin_piece = mv.role();
        let captured_piece = mv.capture().unwrap(); // WENN MICH DAS BUMMST AHH

        let origin_value = piece_value(origin_piece);
        let caputure_value = piece_value(captured_piece);

        score += caputure_value * 10 - origin_value; // Besonders wertvolle eher nehmen
    }
    if mv.is_promotion() {
        score += 1000
    }
    if pos.is_check() {
        score += 10
    }
    score
}

fn negamax(pos: &Chess, depth: u8) -> i32 {
    if depth == 0 || pos.legal_moves().is_empty() {
        return evaluate(pos);
    }

    let mut max = i32::MIN;

    for m in pos.legal_moves() {
        if let Ok(next_pos) = pos.clone().play(m) {
            let score = -negamax(&next_pos, depth - 1);
    
            if score > max {
                max = score
            }
        }
    }

    max
}

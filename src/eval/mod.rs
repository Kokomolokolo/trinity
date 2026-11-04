use shakmaty::{Chess, Color, Position, Role::{self, Bishop, King, Knight, Pawn, Queen, Rook}};

use crate::eval::psqt::{BLACK_PSQT, WHITE_PSQT};

mod psqt;

pub fn evaluate(pos: &Chess) -> i32 {
    let board = pos.board();
    let current_player = pos.turn();
    let mut score = 0;
    
    // Iteriert über das Board und bewertet das Material
    for (sqaure, piece) in board {
        let psqt = if piece.color == Color::White {
            WHITE_PSQT
        } else {
            BLACK_PSQT
        };

        let square_idx = sqaure.to_usize();
        let value = piece_value(piece.role) + match piece.role {
            Role::Pawn => {
                psqt.pawn[square_idx]
            },
            Role::Bishop => {
                psqt.bishop[square_idx]
            },
            Role::Knight => {
                psqt.knight[square_idx]
            },
            Role::Queen => {
                psqt.queen[square_idx]
            },
            Role::Rook => {
                psqt.rook[square_idx]
            },
            Role::King => psqt.king[square_idx], // Wird hier eigentlich nicht beachtet
        };

        if piece.color == shakmaty::Color::White {
            score += value;
        } else {
            score -= value;
        }
    }

    // Umdrehen je nach Farbe
    if current_player == shakmaty::Color::Black {
        -score
    } else {
        score
    }
}

pub fn piece_value(role: Role) -> i32 {
    match role {
        Pawn => 100,
        Knight => 300,
        Bishop => 350,
        Rook => 500,
        Queen => 900,
        King => 0, // Nicht beachtet
    }
}
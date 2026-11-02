use shakmaty::{Chess, Move, Position};

pub fn search(
    pos: &Chess,
    depth: u8,
) -> Option<Move> {
    // Erstmal den ergen move
    let first_mv = pos.legal_moves().first().copied();
    first_mv
}

fn negamax()
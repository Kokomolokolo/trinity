use shakmaty::{Chess, Move};

use crate::search::search;

pub struct Engine {
    // Suchparameter
    max_search_depth: u8,
}

impl Engine {
    pub fn new() -> Self {
        Self { 
            max_search_depth: 4
        }
    }

    // Das Ding
    pub fn find_best_move(
        &mut self,
        pos: &Chess,
    ) -> Option<Move> {
        search(pos, self.max_search_depth)
    }
}
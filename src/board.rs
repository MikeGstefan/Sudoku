use core::fmt;
use std::fmt::Display;

use crate::cell::{self, Cell};



pub struct Board{
    board: [cell::Cell; 81],
}

impl Board{
    pub fn new() -> Board{
        let new_board = [cell::Cell::new();81];


        Board{
            board: new_board,
        }
    }
}

impl Display for Board{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut board_str = String::new();
        for (i, cell) in self.board.iter().enumerate(){
            board_str.push_str(&cell.to_string());
            if i % 9 == 8{
                board_str.push_str("\n");
            }
        }
        write!(f, "{}", board_str)
    }
}
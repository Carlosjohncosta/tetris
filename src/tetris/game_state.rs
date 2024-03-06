use std::collections::VecDeque;

use super::*;
use nannou::{color, rand, rand::seq::SliceRandom};
type Board = Box<[BoardRow]>;

pub enum Axis {
    X,
    Y,
}

#[derive(Clone)]
pub struct BoardRow {
    full: bool,
    row: Box<[Option<color::Rgb<u8>>]>,
}

impl BoardRow {
    fn new(width: usize) -> Self {
        Self {
            full: false,
            row: vec![None; width].into_boxed_slice(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    pub fn get_row(&self) -> &Box<[Option<color::Rgb<u8>>]> {
        &self.row
    }
}

pub struct GameState {
    width: u32,
    height: u32,
    time: u64,
    game_speed: u64,
    pieces: Vec<Piece>,
    current_piece: Piece,
    board: Board,
    break_frame: Option<u32>,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        let board = vec![BoardRow::new(width as usize); height as usize].into_boxed_slice();
        let pieces = Piece::get_standard_pieces();
        let current_piece = Self::get_random_piece(&pieces);
        GameState {
            width,
            height,
            time: 0u64,
            game_speed: 22,
            pieces,
            current_piece,
            board,
            break_frame: None,
        }
    }

    pub fn get_random_piece(pieces: &Vec<Piece>) -> Piece {
        pieces.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn inc_time(&mut self) {
        self.time += 1
    }

    pub fn get_current_piece(&self) -> &Piece {
        &self.current_piece
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_break_frame(&self) -> &Option<u32> {
        &self.break_frame
    }

    // Checks if passed in piece is outised the bounds of the board and if intersecting a block on the grid.
    fn does_intersect(&self, piece: &Piece) -> bool {
        let block_positions = piece.get_block_positions();
        block_positions
            .iter()
            .find(|Point { x, y }| {
                let in_board = (0..self.width as i32).contains(&(*x as i32))
                    && (0..self.height as i32).contains(&(*y as i32));
                if in_board {
                    self.board[*y as usize].row[*x as usize].is_some()
                } else {
                    true
                }
            })
            .is_some()
    }

    pub fn move_current_piece(&mut self, axis: Axis, ammount: f32) -> bool {
        let mut new_piece = self.current_piece.clone();
        match axis {
            Axis::X => new_piece.center.x += ammount,
            Axis::Y => new_piece.center.y += ammount,
        }
        let did_intersect = self.does_intersect(&new_piece);
        if !did_intersect {
            self.current_piece = new_piece;
        }
        !did_intersect
    }

    pub fn rotate_current_piece(&mut self, direction: Direction) {
        let mut new_piece = self.current_piece.clone();
        new_piece.rotate(direction);
        if !self.does_intersect(&new_piece) {
            self.current_piece = new_piece;
        }
    }

    //Checks if any rows are full, and sets BoardRow.full = true if so. Returns true if any are found.
    fn check_full_rows(&mut self) -> bool {
        self.board.iter_mut().fold(false, |full_row_found, row| {
            let curr_row_full = row.row.iter().find(|cell| cell.is_none()).is_none();
            row.full = curr_row_full;
            curr_row_full || full_row_found
        })
    }

    //Removes full rows after break animation is finished.
    fn break_rows(&mut self) {
        //Keeps a queue of empty rows for next rows to go into.
        let mut free_rows: VecDeque<&mut BoardRow> = VecDeque::new();

        //Steps through each row and s
        for row in &mut self.board.iter_mut() {
            if row.is_full() {
                free_rows.push_back(row);
            } else {
                if let Some(free_row) = free_rows.pop_front() {
                    *free_row = BoardRow::new(self.width as usize);
                    std::mem::swap(free_row, row);
                    free_rows.push_back(row);
                }
            }
        }
    }

    pub fn next_frame(&mut self) {
        //Used to count tick of row break animation, and breaks the rows once animation finished.
        if let Some(break_frame) = &mut self.break_frame {
            if *break_frame == 0 {
                self.break_frame = None;
                self.break_rows();
            } else {
                *break_frame -= 1;
            }
            return;
        }

        //Drops piece by one if time is multiple of game_speed.
        if self.time % self.game_speed == 0 {
            let did_intersect = !self.move_current_piece(Axis::Y, -1.0);
            if did_intersect {
                for Point { x, y } in self.current_piece.get_block_positions().iter() {
                    self.board[*y as usize].row[*x as usize] = Some(self.current_piece.color);
                }
                self.current_piece = Self::get_random_piece(&self.pieces);
            }
        }

        if self.check_full_rows() {
            self.break_frame = Some(30);
        }

        self.inc_time();
    }
}

use rand::{self, seq::SliceRandom, Fill};

#[derive(Debug)]
pub struct Puzzle {
    board: Vec<Vec<i16>>,
    empty_cell: (usize, usize),
}

impl Puzzle {
    pub fn new(n: i16) -> Self {
        let mut random_values: Vec<i16> = (0..n * n).collect();
        random_values.shuffle(&mut rand::thread_rng());
        let mut iter = random_values.chunks(n as usize);

        let mut empty_cell: (usize, usize) = (0, 0);

        let board = (0..n)
            .map(|row| {
                let row_values = iter.next().unwrap().to_vec();
                let exists = row_values.iter().position(|v| *v == 0);

                match exists {
                    Some(col) => empty_cell = (row.try_into().unwrap(), col),
                    None => {}
                }

                row_values
            })
            .collect::<Vec<_>>();

        Self { board, empty_cell }
    }

    pub fn get_board(&self) -> &Vec<Vec<i16>> {
        &self.board
    }

    pub fn do_move(&mut self, from: (usize, usize)) {
        if from.0 > self.board.len()
            || from.1 > self.board.len()
            || from.0.abs_diff(self.empty_cell.0) + from.1.abs_diff(self.empty_cell.1) != 1
        {
            return;
        }

        let to = self.empty_cell;
        let temp = self.board[from.0][from.1];
        self.board[from.0][from.1] = self.board[to.0][to.1];
        self.board[to.0][to.1] = temp;
        self.empty_cell = from;
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Neg};

    use super::*;

    #[test]
    fn create_game_and_move() {
        let mut game = Puzzle::new(4);
        let empty = game.empty_cell;
        let click = (
            if empty.0 == 0 {
                empty.0 + 1
            } else {
                empty.0 - 1
            },
            empty.1,
        );

        let v = game.board[click.0][click.1];
        game.do_move(click);

        assert_eq!(game.board[empty.0][empty.1], v);
        assert_eq!(game.empty_cell, click);
    }

    #[test]
    fn move_illegal() {
        let mut game = Puzzle::new(4);
        let empty = game.empty_cell;
        let click = (
            if empty.0 == 0 {
                empty.0 + 1
            } else {
                empty.0 - 1
            },
            if empty.1 == 0 {
                empty.1 + 1
            } else {
                empty.1 - 1
            },
        );
        game.do_move(click);
        assert_eq!(game.empty_cell, empty);
    }
}

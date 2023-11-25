use lazy_static::lazy_static;
use std::collections::HashSet;
const BLOCK_DIMS: usize = 3;
const BLOCKS: usize = BLOCK_DIMS.pow(2);
const BLOCK_SIZE: usize = BLOCK_DIMS.pow(2);
const BLOCK_ROW_SIZE: usize = BLOCK_SIZE * BLOCK_DIMS;
const _BLOCK_COL_SIZE: usize = BLOCK_ROW_SIZE;
const GRID_DIMS: usize = BLOCK_DIMS.pow(2);
const GRID_SIZE: usize = GRID_DIMS.pow(2);

pub const HIGHEST_SUDOKU_NUMBER: u8 = (BLOCK_DIMS as u8).pow(2);

lazy_static! {
    static ref SUDOKU_NUMBERS: HashSet<u8> = {
        let mut numbers = HashSet::new();
        for num in 1..=HIGHEST_SUDOKU_NUMBER {
            numbers.insert(num);
        }
        numbers
    };
}

#[derive(Clone, Debug)]
pub struct SudokuPuzzle {
    pub grid: [u8; GRID_SIZE],
    rows: [[u8; GRID_DIMS]; GRID_DIMS],
    blocks: [[u8; BLOCK_SIZE]; BLOCKS],
    cols: [[u8; GRID_DIMS]; GRID_DIMS],
    move_stack: Vec<usize>,
}

impl PartialEq for SudokuPuzzle {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid && self.rows == other.rows && self.blocks == other.blocks
    }
}

impl SudokuPuzzle {
    pub fn from_array(grid: [u8; GRID_SIZE]) -> SudokuPuzzle {
        let rows = get_rows_from_grid(grid);
        let blocks = get_blocks_from_grid(grid);
        let cols = get_cols_from_grid(grid);
        SudokuPuzzle {
            grid,
            rows,
            blocks,
            cols,
            move_stack: Vec::new(),
        }
    }

    pub fn from_vec(grid: Vec<u8>) -> SudokuPuzzle {
        let mut target_grid: [u8; GRID_SIZE] = [0; GRID_SIZE];
        target_grid.clone_from_slice(&grid);

        SudokuPuzzle::from_array(target_grid)
    }

    pub fn from_str(s: &str) -> SudokuPuzzle {
        let grid = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        SudokuPuzzle::from_vec(grid)
    }

    pub fn set_square(&mut self, index: usize, value: u8) {
        self.grid[index] = value;
        self.set_square_in_row(index, value);
        self.set_square_in_block(index, value);
        self.set_square_in_col(index, value);

        self.move_stack.push(index)
    }

    pub fn undo_move(&mut self) {
        let index = self.move_stack.pop().unwrap();

        self.grid[index] = 0;
        self.set_square_in_row(index, 0);
        self.set_square_in_block(index, 0);
        self.set_square_in_col(index, 0);
    }

    pub fn undo_moves(&mut self, moves_to_undo: usize) {
        for _ in 0..moves_to_undo {
            self.undo_move();
        }
    }

    pub fn is_solved(&self) -> bool {
        self.grid.into_iter().all(|square| square != 0)
    }

    pub fn get_possible_clues(&self, index: usize) -> Vec<u8> {
        let row_numbers = self.get_row(index);
        let col_numbers = self.get_col(index);
        let block_numbers = self.get_block(index);

        let mut all_seen_numbers = [row_numbers, col_numbers, block_numbers].concat();
        all_seen_numbers.sort_unstable();
        all_seen_numbers.dedup();
        if all_seen_numbers[0] == 0 {
            all_seen_numbers.swap_remove(0);
        }
        let all_seen_numbers: HashSet<_> = all_seen_numbers.into_iter().collect();

        SUDOKU_NUMBERS.difference(&all_seen_numbers).map(|x| *x).collect()
    }

    pub fn get_clueless_squares_enumerated(&self) -> impl Iterator<Item = usize> {
        self.grid
            .into_iter()
            .enumerate()
            .filter(|(_, square)| *square == 0)
            .map(|(i, _)| i)
    }

    pub fn get_next_clueless_index(&self) -> Option<usize> {
        self.grid.iter().position(|x| *x == 0)
    }

    pub fn get_row(&self, index: usize) -> [u8; GRID_DIMS] {
        let row_index = get_row_index(index);
        self.rows[row_index]
    }

    pub fn get_block(&self, index: usize) -> [u8; BLOCK_SIZE] {
        let block_index = get_block_index(index);
        self.blocks[block_index]
    }

    pub fn get_col(&self, index: usize) -> [u8; GRID_DIMS] {
        let column_index = get_column_index(index);
        self.cols[column_index]
    }

    fn set_square_in_row(&mut self, index: usize, value: u8) {
        let row_index = get_row_index(index);
        let index_in_row = get_index_in_row(index);
        self.rows[row_index][index_in_row] = value
    }

    fn set_square_in_block(&mut self, index: usize, value: u8) {
        let block_index = get_block_index(index);
        let index_in_block = get_index_in_block(index);

        self.blocks[block_index][index_in_block] = value;
    }

    fn set_square_in_col(&mut self, index: usize, value: u8) {
        let column_index = get_column_index(index);
        let index_in_column = get_index_in_column(index);

        self.cols[column_index][index_in_column] = value;
    }
    pub fn moves_count(&self) -> usize {
        self.move_stack.len()
    }
}

fn get_rows_from_grid(grid: [u8; GRID_SIZE]) -> [[u8; GRID_DIMS]; GRID_DIMS] {
    let mut rows = [[0; GRID_DIMS]; GRID_DIMS];

    for (i, &square) in grid.iter().enumerate() {
        let row_index = get_row_index(i);
        let index_in_row = get_index_in_row(i);

        rows[row_index][index_in_row] = square;
    }
    rows
}

fn get_blocks_from_grid(grid: [u8; GRID_SIZE]) -> [[u8; BLOCK_SIZE]; BLOCKS] {
    let mut blocks = [[0; BLOCK_SIZE]; BLOCKS];

    for (i, &square) in grid.iter().enumerate() {
        let block_index = get_block_index(i);
        let index_in_block = get_index_in_block(i);

        blocks[block_index][index_in_block] = square;
    }
    blocks
}

fn get_cols_from_grid(grid: [u8; GRID_SIZE]) -> [[u8; GRID_DIMS]; GRID_DIMS] {
    let mut cols = [[0; GRID_DIMS]; GRID_DIMS];

    for (i, &square) in grid.iter().enumerate() {
        let column_index = get_column_index(i);
        let index_in_column = get_index_in_column(i);

        cols[column_index][index_in_column] = square;
    }
    cols
}

fn get_row_index(index: usize) -> usize {
    index / GRID_DIMS
}

fn get_index_in_row(index: usize) -> usize {
    index % GRID_DIMS
}

fn get_block_index(index: usize) -> usize {
    let block_row = index / BLOCK_ROW_SIZE;
    let block_col = (index % GRID_DIMS) / BLOCK_DIMS;

    block_col + block_row * BLOCK_DIMS
}

fn get_index_in_block(index: usize) -> usize {
    let row_in_block = (index / GRID_DIMS) % BLOCK_DIMS;
    let col_in_block = index % BLOCK_DIMS;

    col_in_block + row_in_block * BLOCK_DIMS
}

fn get_column_index(index: usize) -> usize {
    index % GRID_DIMS
}

fn get_index_in_column(index: usize) -> usize {
    index / GRID_DIMS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_puzzle() -> SudokuPuzzle {
        let grid_values: Vec<u8> = (0..81).into_iter().collect();
        SudokuPuzzle::from_array(grid_values.try_into().unwrap())
    }
    #[test]
    fn setting_first_square_updates_all_states() {
        let mut puzzle = get_test_puzzle();

        puzzle.set_square(0, 100);

        assert_eq!(puzzle.grid[0], 100);
        assert_eq!(puzzle.rows[0][0], 100);
        assert_eq!(puzzle.blocks[0][0], 100);
        assert_eq!(puzzle.cols[0][0], 100);
    }
    #[test]
    fn setting_62nd_square_updates_all_states() {
        let mut puzzle = get_test_puzzle();

        puzzle.set_square(61, 161);

        assert_eq!(puzzle.grid[61], 161);
        assert_eq!(puzzle.rows[6][7], 161);
        assert_eq!(puzzle.blocks[8][1], 161);
        assert_eq!(puzzle.cols[7][6], 161);
    }
    #[test]
    fn getting_block_for_index_35_gives_correct_values() {
        let puzzle = get_test_puzzle();

        let block = puzzle.get_block(35);

        let expected_block = [33, 34, 35, 42, 43, 44, 51, 52, 53];

        assert_eq!(block, expected_block);
    }

    #[test]
    fn getting_row_for_index_47_gives_correct_values() {
        let puzzle = get_test_puzzle();

        let row = puzzle.get_row(47);

        let expected_row = [45, 46, 47, 48, 49, 50, 51, 52, 53];

        assert_eq!(row, expected_row);
    }

    #[test]
    fn getting_col_for_index_14_gives_correct_values() {
        let puzzle = get_test_puzzle();

        let col = puzzle.get_col(14);

        let expected_col = [5, 14, 23, 32, 41, 50, 59, 68, 77];

        assert_eq!(col, expected_col);
    }

    #[test]
    fn undo_move_returns_board_to_previous_position() {
        let mut puzzle = get_test_puzzle();

        puzzle.set_square(0, 100);

        puzzle.undo_move();

        let expected_result = get_test_puzzle();

        assert_eq!(puzzle, expected_result);
    }
}

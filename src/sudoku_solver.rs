use crate::sudoku_grid::SudokuPuzzle;

// Returns whether the puzzle was solved
pub fn solve(puzzle: &mut SudokuPuzzle) -> bool {
    let moves_before_fill = puzzle.moves_count();
    fill_forced_clues(puzzle);
    let force_filled_moves = puzzle.moves_count() - moves_before_fill;
    if let Some(i) = puzzle.get_next_clueless_index() {
        for possible_clue in puzzle.get_possible_clues(i) {
            puzzle.set_square(i, possible_clue);
            if puzzle.is_solved() {
                return true;
            } else {
                if solve(puzzle) {
                    return true;
                }
            }
        }
        puzzle.undo_moves(force_filled_moves + 1);
        return false;
    }
    true
}

fn fill_forced_clues(puzzle: &mut SudokuPuzzle) {
    let mut found_clue = true;
    while found_clue {
        found_clue = forced_fill(puzzle);
    }
}

fn forced_fill(puzzle: &mut SudokuPuzzle) -> bool {
    let mut filled_square: bool = false;
    for i in puzzle.get_clueless_squares_enumerated() {
        if let Some(clue) = get_forced_clue(puzzle, i) {
            puzzle.set_square(i, clue);
            filled_square = true;
        }
    }
    filled_square
}

fn get_forced_clue(puzzle: &SudokuPuzzle, index: usize) -> Option<u8> {
    let possible_clues = puzzle.get_possible_clues(index);
    match possible_clues.len() == 1 {
        true => Some(possible_clues[0]),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku_grid::SudokuPuzzle;

    use super::*;

    #[test]
    fn solver_solves_sudoku_with_one_missing_in_each_direction() {
        #[rustfmt::skip]
        let puzzle_grid = [
            0, 9, 4, 8, 6, 5, 2, 3, 7, 
            7, 0, 5, 4, 1, 2, 9, 6, 8, 
            8, 6, 0, 3, 9, 7, 1, 4, 5, 
            9, 2, 1, 0, 4, 8, 3, 5, 6, 
            6, 7, 8, 5, 0, 1, 4, 2, 9, 
            4, 5, 3, 9, 2, 0, 8, 7, 1, 
            3, 8, 9, 6, 5, 4, 0, 1, 2, 
            2, 4, 6, 1, 7, 9, 5, 0, 3, 
            5, 1, 7, 2, 8, 3, 6, 9, 0,
        ];

        let mut puzzle = SudokuPuzzle::from_array(puzzle_grid);

        solve(&mut puzzle);

        #[rustfmt::skip]
        let expected_puzzle_grid = [
            1, 9, 4, 8, 6, 5, 2, 3, 7, 
            7, 3, 5, 4, 1, 2, 9, 6, 8, 
            8, 6, 2, 3, 9, 7, 1, 4, 5, 
            9, 2, 1, 7, 4, 8, 3, 5, 6, 
            6, 7, 8, 5, 3, 1, 4, 2, 9, 
            4, 5, 3, 9, 2, 6, 8, 7, 1, 
            3, 8, 9, 6, 5, 4, 7, 1, 2, 
            2, 4, 6, 1, 7, 9, 5, 8, 3, 
            5, 1, 7, 2, 8, 3, 6, 9, 4,
        ];

        assert_eq!(expected_puzzle_grid, puzzle.grid);
    }

    #[test]
    fn solver_solves_sudoku_with_multiple_numbers_missing_in_each_direction() {
        #[rustfmt::skip]
        let puzzle_grid = [
            0, 7, 0, 0, 0, 5, 0, 0, 0, 
            1, 0, 0, 0, 3, 0, 5, 0, 8, 
            0, 0, 0, 2, 0, 9, 0, 6, 0, 
            9, 1, 0, 5, 0, 0, 4, 2, 0, 
            6, 8, 0, 3, 0, 0, 0, 1, 0, 
            2, 5, 4, 0, 9, 0, 0, 0, 3, 
            7, 0, 6, 8, 0, 1, 0, 4, 0, 
            3, 4, 5, 0, 0, 6, 0, 7, 1, 
            0, 0, 1, 0, 7, 0, 2, 0, 6,
        ];

        let mut puzzle = SudokuPuzzle::from_array(puzzle_grid);

        solve(&mut puzzle);

        #[rustfmt::skip]
        let expected_puzzle_grid = [
            4, 7, 9, 6, 8, 5, 1, 3, 2, 
            1, 6, 2, 7, 3, 4, 5, 9, 8, 
            5, 3, 8, 2, 1, 9, 7, 6, 4, 
            9, 1, 3, 5, 6, 8, 4, 2, 7, 
            6, 8, 7, 3, 4, 2, 9, 1, 5, 
            2, 5, 4, 1, 9, 7, 6, 8, 3, 
            7, 2, 6, 8, 5, 1, 3, 4, 9, 
            3, 4, 5, 9, 2, 6, 8, 7, 1,
            8, 9, 1, 4, 7, 3, 2, 5, 6,
        ];

        assert_eq!(expected_puzzle_grid, puzzle.grid);
    }
}

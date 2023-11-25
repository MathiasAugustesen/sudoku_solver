#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use sudoku_solver::{solve, SudokuPuzzle};
    struct TestCase {
        input: SudokuPuzzle,
        solution: [u8; 81],
    }

    #[derive(Deserialize, Debug)]
    struct TestCaseDto {
        input: Vec<u8>,
        solved: Vec<u8>,
    }
    fn get_solved_sudokus(sudokus: &str) -> Vec<TestCase> {
        let mut result = Vec::new();
        let test_case_dtos: Vec<TestCaseDto> = serde_json::from_str(sudokus).unwrap();
        for case in test_case_dtos {
            result.push(TestCase {
                input: SudokuPuzzle::from_vec(case.input),
                solution: case.solved.try_into().unwrap()
            })
        }
        result
    }

    fn get_medium_solved_sudokus() -> Vec<TestCase> {
        let sudokus = include_str!("../sudokus/solved_medium.json");
        get_solved_sudokus(sudokus)
    }

    fn get_hard_solved_sudokus() -> Vec<TestCase> {
        let sudokus = include_str!("../sudokus/solved_hard.json");
        get_solved_sudokus(sudokus)
    }

    fn get_diabolical_sudokus() -> Vec<TestCase> {
        let sudokus = include_str!("../sudokus/solved_diabolical.json");
        get_solved_sudokus(sudokus)
    }
    

    #[test]
    fn medium_sudokus_are_solved() {
        let test_cases = get_medium_solved_sudokus();

        for TestCase { mut input, solution} in test_cases {
            solve(&mut input);
            assert_eq!(input.grid, solution);
        }
    }

    #[test]
    fn hard_sudokus_are_solved() {
        let test_cases = get_hard_solved_sudokus();

        for TestCase { mut input, solution} in test_cases {
            solve(&mut input);
            assert_eq!(input.grid, solution);
        }
    }

    #[test]
    fn diabolical_sudokus_are_solved() {
        let test_cases = get_diabolical_sudokus();

        for TestCase { mut input, solution} in test_cases {
            solve(&mut input);
            assert_eq!(input.grid, solution);
        }
    }
}

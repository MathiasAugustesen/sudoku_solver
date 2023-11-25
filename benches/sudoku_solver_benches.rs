use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku_solver::{solve, SudokuPuzzle};

fn get_solved_sudokus(sudokus: &str) -> Vec<SudokuPuzzle> {
    let mut result = Vec::new();
    let test_case_dtos: Vec<Vec<u8>> = serde_json::from_str(sudokus).unwrap();
    for case in test_case_dtos {
        result.push(SudokuPuzzle::from_vec(case))
    }
    result
}

fn get_medium_bench_sudokus() -> Vec<SudokuPuzzle> {
    let sudokus = include_str!("../sudokus/medium.json");
    get_solved_sudokus(sudokus)
}

fn get_hard_bench_sudokus() -> Vec<SudokuPuzzle> {
    let sudokus = include_str!("../sudokus/hard.json");
    get_solved_sudokus(sudokus)
}

fn get_diabolical_bench_sudokus() -> Vec<SudokuPuzzle> {
    let sudokus = include_str!("../sudokus/diabolical.json");
    get_solved_sudokus(sudokus)
}

fn medium_sudoku_benchmark(c: &mut Criterion) {
    let puzzles = get_medium_bench_sudokus();

    c.bench_function("solve medium sudokus", |b| {
        b.iter(|| {
            for puzzle in &puzzles {
                black_box(solve(&mut puzzle.clone()));
            }
        })
    });
}

fn hard_sudoku_benchmark(c: &mut Criterion) {
    let puzzles = get_hard_bench_sudokus();

    c.bench_function("solve hard sudokus", |b| {
        b.iter(|| {
            for puzzle in &puzzles {
                black_box(solve(&mut puzzle.clone()));
            }
        })
    });
}

fn diabolical_sudoku_benchmark(c: &mut Criterion) {
    let puzzles = get_diabolical_bench_sudokus();

    c.bench_function("solve diabolical sudokus", |b| {
        b.iter(|| {
            for puzzle in &puzzles {
                black_box(solve(&mut puzzle.clone()));
            }
        })
    });
}

fn impossible_sudoku_benchmark(c: &mut Criterion) {
    let puzzle = SudokuPuzzle::from_vec(vec![
        0,0,0,4,0,1,0,3,0,
        1,0,7,0,8,0,0,0,2,
        0,5,0,0,0,0,0,0,0,
        0,8,0,0,0,0,0,0,5,
        5,0,9,0,0,3,0,2,0,
        0,6,0,0,9,0,0,0,0,
        9,0,2,0,7,0,0,0,1,
        0,0,6,0,0,0,0,0,0,
        0,0,0,0,0,8,7,0,0,
    ]);

    c.bench_function("solve impossible sudoku", |b| {
        b.iter(|| {
                black_box(solve(&mut puzzle.clone()));
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = impossible_sudoku_benchmark, medium_sudoku_benchmark, hard_sudoku_benchmark, diabolical_sudoku_benchmark
}
criterion_main!(benches);

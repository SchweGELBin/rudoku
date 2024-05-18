use grid::*;
use rand::{thread_rng, seq::SliceRandom, Rng};

fn main() {
    // Example grid
    let grid: Grid<u8> = grid![
        [1,2,3,0,0,0,0,0,0]
        [0,0,0,4,5,6,0,0,0]
        [0,0,0,0,0,0,7,8,9]
        [0,0,0,0,0,0,0,0,0]
        [0,0,0,0,0,0,0,0,0]
        [0,0,0,0,0,0,0,0,0]
        [0,0,0,0,0,0,0,0,0]
        [0,0,0,0,0,0,0,0,0]
        [0,0,0,0,0,0,0,0,0]
    ];
    println!("Example grid:");
    print_grid(grid.clone());

    // Solve grid
    let solved_grid = solve(grid.clone(), None).
        expect("Sudoku unsolvable");
    println!("Solved grid:");
    print_grid(solved_grid.clone());

    // Generate grid
    let generated_grid = generate();
    println!("Generated grid:");
    print_grid(generated_grid.clone());
}

// Checks if the given number is valid in the given cell
fn check_validity(grid: Grid<u8>, row: u8, col: u8, num: u8) -> bool {
    // Check Row
    for i in 0..9 {
        if grid.get(row, i) == Some(&num) {
            return false;
        }
    }
    // Check Column
    for i in 0..9 {
        if grid.get(i, col) == Some(&num) {
            return false;
        }
    }
    // Check Square
    for i in 0..3 {
        for j in 0..3 {
            if grid.get(row - row % 3 + i, col - col % 3 + j) == Some(&num) {
                return false;
            }
        }
    }
    // Else: Valid
    true
}

// Solves the sudoku recursively
fn solve(mut grid: Grid<u8>, seq: Option<Vec<u8>>) -> Option<Grid<u8>> {
    let mut seq: Vec<u8> = seq.unwrap_or((0..81).collect());
    // Finished generating
    if seq.is_empty() {
        return Some(grid);
    }
    // Select cell, based on sequence
    let col: u8 = seq[0] % 9;
    let row: u8 = (seq[0] - col) / 9;
    // Remove used element from sequence
    seq.remove(0);
    // Skip set cells
    if grid.get(row, col) > Some(&0) {
        return solve(grid.clone(), Some(seq.clone()));
    }
    // Test cell
    for i in 1..=9 {
        if check_validity(grid.clone(), row, col, i) {
            // Set element to i
            let grid = set_element(grid.clone(), row, col, i);
            // Continue recursively
            let solved_grid = solve(grid.clone(), Some(seq.clone()));
            if solved_grid.is_some() {
                return Some(solved_grid?);
            }
        }
        // Something prior was invalid
        grid = set_element(grid, row, col, 0);
    }
    // Else: No possible solution
    None
}

// Sets the given number in the given cell
fn set_element(grid: Grid<u8>, row: u8, col: u8, num: u8) -> Grid<u8> {
    let row: usize = row.into();
    let col: usize = col.into();
    let mut grid = grid;
    let mut tmp: Vec<u8> = grid.remove_row(row).
        expect("Couldn't set element: Row not found");
    tmp[col] = num;
    grid.insert_row(row, tmp);
    grid
}

// Prints the given sudoku
fn print_grid(grid: Grid<u8>) {
    let mut j = 0;
    let line = format!(" {}","-".repeat(7)).repeat(3);
    for i in grid.iter() {
        // Format
        if j % 9 == 0 && j != 0 {
            println!("|");
        }
        if j % 27 == 0 {
            println!("{line} ");
        }
        if j % 3 == 0 {
            print!("| ");
        }
        // Print value
        if *i != 0 {
            print!("{i} ");
        } else {
            print!("  ");
        }
        j += 1;
    }
    println!("|\n{line} ");
}

// Generates a sudoku
fn generate() -> Grid<u8>{
    // Set empty grid
    let mut start_grid: Grid<u8> = Grid::new(9,9);
    // Set sequence
    let mut seq: Vec<u8> = (0..81).collect();
    let mut rem: Vec<u8> = Vec::new();
    // Start with sector 1, 5, 9
    for _i in 0..3 {
        let mut sec: Vec<u8> = (1..=9).collect();
        sec.shuffle(&mut thread_rng());
        for _row in 0..3 {
            for _col in 0..3 {
                start_grid = set_element(start_grid.clone(), _row + _i*3, _col + _i*3, sec[0]);
                sec.remove(0);
                rem.push(_col+_i*3+(_row+_i*3)*9); 
            }
        }
    }
    // Shorten sequence
    rem.reverse();
    for i in rem.iter() {
        seq.remove((*i).into());
    }
    // Shuffle sequence
    seq.shuffle(&mut thread_rng()); 
    // Solve grid with random sequence
    let mut grid = solve(start_grid.clone(), Some(seq.clone())).
        expect("Sudoku unsolvable");
    // Print solution
    println!("Solution of generated grid:");
    print_grid(grid.clone()); 
    // Remove some cells
    let miss = thread_rng().gen_range(27..=45);
    let mut seq_miss: Vec<u8> = (0..81).collect();
    seq_miss.shuffle(&mut thread_rng());
    for _i in 0..=miss {
        let col: u8 = seq_miss[0] % 9;
        let row: u8 = (seq_miss[0] - col) / 9;
        grid = set_element(grid.clone(), row, col, 0);
        seq_miss.remove(0);
    }
    // Return grid
    grid
}

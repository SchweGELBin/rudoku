use grid::*;

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

    // Solve grid
    let solved_grid = solve(grid.clone(), 0, 0).
        expect("Sudoku unsolvable"); 
    print_grid(solved_grid.clone()); 
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
fn solve(mut grid: Grid<u8>, mut row: u8, mut col: u8) -> Option<Grid<u8>> { 
    // Handle overflow
    if col >= 9 {
        col = 0;
        row += 1;
    }
    // Finished solving
    if row >= 9 {
        return Some(grid);
    }
    // Skip set cells
    if grid.get(row, col) > Some(&0) { 
        return solve(grid.clone(), row, col + 1);
    } 
    // Test cell
    for i in 1..=9 {
        if check_validity(grid.clone(), row, col, i) {
            // Set element to i
            let grid = set_element(grid.clone(), row, col, i); 
            // Continue recursively
            let solved_grid = solve(grid.clone(), row, col + 1);
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

fn print_grid(grid: Grid<u8>) {
    for i in 0..grid.rows() {
        if i % 3 == 0 {
            println!();
        }
        for j in 0..grid.cols() {
            if j % 3 == 0 {
                print!("  ");
            }
            print!("{}", grid.get(i, j).
                   expect("Couldn't print grid: Cell not found")); 
        }
        println!();
    }
}

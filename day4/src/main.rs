use std::fs;

const FILENAME: &str = "./input.txt";
const ADJACENT_POSITIONS: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
const PAPER_CHAR: char = '@';
const SPACE_CHAR: char = '.';

fn main() {
    let grid = parse_input();
    let height = grid.len();
    let width = grid[0].len();

    let mut total = 0;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == PAPER_CHAR && count_adjacent(&grid, i, j) < 4 {
                total += 1;
            }
        }
    }

    println!("Can remove {} rolls", total);
    println!("Removed {} rolls", remove_rolls());
}

fn remove_rolls () -> usize {
    let mut grid = parse_input();
    let height = grid.len();
    let width = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut removed = 0;

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] == PAPER_CHAR && count_adjacent(&grid, i, j) < 4 {
                    grid[i][j] = SPACE_CHAR;
                    removed += 1;
                    total_removed += 1;
                }
            }
        }

        if removed == 0 {
            break;
        }
    }
    return total_removed;
}

fn count_adjacent (grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for adjacent_pos in ADJACENT_POSITIONS {
        let adjacent_cell = get_adjacent_coords(height, width, i, j, adjacent_pos).map(|coords| grid[coords.0][coords.1]);
        if adjacent_cell.is_some() && adjacent_cell.unwrap() == PAPER_CHAR{
            count += 1;
        }
    }

    return count;
}

fn get_adjacent_coords (height: usize, width: usize, i: usize, j: usize, adjacent_pos: (i32, i32)) -> Option<(usize, usize)> {
    let x_coord = i as i32 + adjacent_pos.0;
    let y_coord = j as i32 + adjacent_pos.1;

    if (x_coord) < 0 || (x_coord) >= height as i32 || (y_coord) < 0 || (y_coord) >= width as i32 {
        return None;
    }

    return Some((x_coord as usize, y_coord as usize));
    
}

fn parse_input () -> Vec<Vec<char>> {
    let mut rolls = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        rolls.push(
            line.chars().collect::<Vec<char>>()
        );
    }
    return rolls;
}
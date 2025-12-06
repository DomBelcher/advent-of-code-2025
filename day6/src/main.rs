use std::fs;

const FILENAME: &str = "./input.txt";

const ADDITION: char = '+';
const MULTIPLICATION: char = '*';
const WHITESPACE: char = ' ';

fn main() {
    let problems = parse_input();
    let mut total = 0;

    for problem in problems {
        let ans = solve(problem);
        total += ans.unwrap();
    }

    let problem_grid = parse_input_grid();
    let part_2_problems = grid_to_problems(problem_grid);

    let mut total_2 = 0;
    for problem in part_2_problems {
        let ans = solve(problem);
        total_2 += ans.unwrap();
        println!("{}", ans.unwrap());
    }

    println!("Total: {}", total);
    println!("Total 2: {}", total_2);
}

fn solve (problem: Problem) -> Option<i64> {
    if problem.operator == ADDITION {
        let mut total = 0;
        for op in problem.operands {
            total += op;
        }
        return Some(total);
    } else if problem.operator == MULTIPLICATION {
        let mut total = 1;
        for op in problem.operands {
            total *= op;
        }
        return Some(total);
    }

    return None
}

fn parse_input_grid () -> Vec<Vec<char>> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect());
    }
    return input
}

fn grid_to_problems (grid: Vec<Vec<char>>) -> Vec<Problem> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut problems = vec![];

    let mut problem = Problem::new(); 
    for col_idx in 0..n_cols {
        if is_separator_column(&grid, col_idx) {
            problems.push(problem);
            problem = Problem::new();
            continue;
        }

        if grid[n_rows-1][col_idx] == MULTIPLICATION {
            problem.operator = MULTIPLICATION
        } else if grid[n_rows-1][col_idx] == ADDITION {
            problem.operator = ADDITION
        }

        problem.operands.push(parse_column_as_int(&grid, col_idx));
    }
    problems.push(problem);

    return problems
}

fn parse_column_as_int (grid: &Vec<Vec<char>>, col_idx: usize) -> i64 {
    let n_rows = grid.len();
    let mut digits = vec![];
    for row_idx in 0..(n_rows-1) {
        let num_char = grid[row_idx][col_idx];
        if num_char != WHITESPACE {
            digits.push(num_char.to_digit(10).unwrap() as i64);
            // total += 10_i64.pow((n_rows-2) as u32) * (num_char.to_digit(10).unwrap() as i64)
        }
    }

    let mut total = 0;
    let n_digits = digits.len();
    for (digit_idx, d) in digits.iter().enumerate() {
        total += 10_i64.pow((n_digits - 1 - digit_idx) as u32) * d;
    }
    return total
}

fn is_separator_column (grid: &Vec<Vec<char>>, col_idx: usize) -> bool {
    let n_rows = grid.len();
    for row_idx in 0..n_rows {
        if grid[row_idx][col_idx] != WHITESPACE {
            return false
        }
    }
    return true
}

fn parse_input () -> Vec<Problem> {
    let mut input_mode = "operands";

    let mut problems = vec![];

    for (idx, line) in fs::read_to_string(FILENAME).unwrap().lines().enumerate() {
        if line.chars().nth(0) == Some(ADDITION) || line.chars().nth(0) == Some(MULTIPLICATION) {
            input_mode = "operators";
        }

        if input_mode == "operands" {
            let line_operands: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
           for (op_idx, operand) in line_operands.iter().enumerate() {
                if idx == 0 {
                    problems.push(Problem {
                        operands: vec![*operand],
                        operator: '-',
                    });
                } else {
                    problems[op_idx].operands.push(*operand);
                }
           }
            
        } else if input_mode == "operators" {
            let line_operators: Vec<char> = line.split_whitespace().map(|n| n.parse::<char>().unwrap()).collect();
            for (op_idx, operator) in line_operators.iter().enumerate() {
                problems[op_idx].operator = *operator;
            }
        }
    }

    return problems;
}

struct Problem {
    operands: Vec<i64>,
    operator: char
}

impl Problem {
    fn new () -> Problem {
        return Problem { operands: vec![], operator: '_' }
    }
}
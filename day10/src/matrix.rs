use std::fmt::{Debug, Formatter};

const VERBOSE: bool = false;


struct Matrix {
    n_rows: usize,
    n_cols: usize,
    data: Vec<Vec<i32>>
}

impl Matrix {
    
}

impl Debug for Matrix {
    fn fmt (&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        for row in self.data.iter() {
            println!("{:?}", row)
        }
        
        return Ok(())
    }
}

pub fn reduce_matrix (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>) {
    return do_reduction(matrix, values, 0);
}


fn do_reduction (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, col_idx: usize) {
    let n_cols = matrix[0].len();

    if col_idx == n_cols {
        // reduced on every column
        return;
    }

    if VERBOSE {
        println!("Current matrix:");
        print_matrix(&matrix);
        println!("{:?}", values);
    }

    if VERBOSE { println!("targeting column {}", col_idx); }

    let pivot_rows = find_source_and_target_row(&matrix, col_idx);
    if pivot_rows.is_none() {
        // not possible to pivot on this column
        // try next col
        return do_reduction(matrix, values, col_idx + 1);
    }
    let (source_row_idx, target_row_idx) = pivot_rows.unwrap();
    if VERBOSE { println!("sourcing from row {}", source_row_idx); }
    if VERBOSE { println!("targeting row {}", target_row_idx); }

    let sign = matrix[target_row_idx][col_idx].signum() * matrix[source_row_idx][col_idx].signum();
    if sign == 0 {
        panic!("zero sign")
    }
    if VERBOSE { println!("sign is {}", sign); }

    if VERBOSE {
        println!("Scaling:");
        println!("before scaling");
        print_matrix(matrix);
    }

    let source_scale_factor = matrix[source_row_idx][col_idx];
    let target_scale_factor =  matrix[target_row_idx][col_idx];

    scale_row_by(matrix, values, source_row_idx, target_scale_factor);
    scale_row_by(matrix, values, target_row_idx, source_scale_factor);

    if VERBOSE {
        println!("after scaling");
        print_matrix(matrix);
    }

    // subtract source row from target row
    for col in 0..n_cols {
        matrix[target_row_idx][col] -= matrix[source_row_idx][col];
    }
    values[target_row_idx] -= values[source_row_idx];
    scale_row(matrix, values, source_row_idx);
    scale_row(matrix, values, target_row_idx);

    return do_reduction(matrix, values, col_idx);
}

fn scale_row_by(matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, row_idx: usize, scale_factor: i32) {
    if VERBOSE { println!("Scaling row {} by factor of {}", row_idx, scale_factor) }
    let n_cols = matrix[row_idx].len();
    for col_idx in 0..n_cols {
        matrix[row_idx][col_idx] *= scale_factor;
    }
    values[row_idx] *= scale_factor;
    return
}

fn scale_row (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, row_idx: usize) {
    let n_cols = matrix[row_idx].len();
    let mut gcd = 1;

    if matrix[row_idx].iter().all(|v| *v <= 0) {
        // values all negative
        // make positive before finding gcd
        scale_row_by(matrix, values, row_idx, -1);
    }

    let min_val = matrix[row_idx].iter().filter(|v| **v != 0).map(|v| v.abs()).min().unwrap_or(1);

    if min_val == 1 {
        return
    }

    for possible_gcd in 2..(min_val + 1) {
        let mut is_cd = true;

        if values[row_idx].abs() % possible_gcd != 0 {
            is_cd = false;
            continue;
        }

        for col_idx in 0..n_cols {
            if matrix[row_idx][col_idx].abs() % possible_gcd != 0 {
                is_cd = false
            }
        }

        if is_cd {
            gcd = possible_gcd;
        }
    }

    if gcd == 1 {
        return
    }

    if VERBOSE { println!("Scaling row {} by factor of {}", row_idx, gcd); }

    for col_idx in 0..n_cols {
        matrix[row_idx][col_idx] /= gcd;
    }
    values[row_idx] /= gcd;
    // panic!();
    return
}

fn find_source_and_target_row (matrix: &Vec<Vec<i32>>, col_idx: usize) -> Option<(usize, usize)> {
    let n_rows = matrix.len();

    for source_row_idx in 0..n_rows {
        if matrix[source_row_idx][col_idx] == 0 {
            continue;
        }

        let mut valid_source = true;
        for c in 0..col_idx {
            if matrix[source_row_idx][c] != 0 {
                valid_source = false;
                break
            }
        }

        if valid_source {
            for target_row_idx in 0..n_rows {
                if matrix[target_row_idx][col_idx] == 0 {
                    continue;
                }

                if source_row_idx != target_row_idx {
                    return Some((source_row_idx, target_row_idx))
                }
            }
        }
    }

    return None
}


pub fn print_matrix (matrix: &Vec<Vec<i32>>) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
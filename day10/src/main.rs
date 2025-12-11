use std::cmp::{max, min};
use std::env::var;
use std::i32;
use std::{collections::HashSet, fs, time::Instant};
use std::iter::FromIterator;

const FILENAME: &str = "./input.txt";
const ON_CHAR: char = '#';
const POWERS_OF_2: [usize; 14] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];

const VERBOSE: bool = false;

fn main() {
    let start = Instant::now();
    let machines = parse_input();
    println!("Input parsed in {}ms", start.elapsed().as_millis());
    println!("{} machines", machines.len());

    let mut total_presses = 0;

    let mut part_2_presses = 0;

    for machine in machines {
        println!("solving for {}", machine.raw);

        let ans = solve_machine(&machine);
        total_presses += ans;
        println!("{}", ans);

        let (matrix, values) = make_matrix(&machine);
        let mut reduced_matrix = matrix.clone();
        let mut reduced_vals = values.clone();
        print_matrix(&matrix);
        // panic!();
        reduce_matrix(&mut reduced_matrix, &mut reduced_vals);

        println!("matrix representation:");
        print_matrix(&reduced_matrix);
        println!("values:");
        println!("{:?}", reduced_vals);
        // panic!();

        let max_pushes = max_button_pushes(&machine);
        println!("Max pushes of each button: {:?}", max_pushes);
        let solved_joltage = solve_joltage(&mut reduced_matrix, &mut reduced_vals, &max_pushes, *values.iter().max().unwrap());
        let joltage_answer = solved_joltage.iter().sum::<i32>();

        if !check_joltage_solution(&machine, &solved_joltage) {
            println!("Solution for machine {} is not valid", machine.raw);
            println!("solution was {:?}", solved_joltage);
            panic!();
        }

        if joltage_answer == i32::MAX {
            println!("failed to solve machine {}", machine.raw);
            println!("matrix representation:");
            print_matrix(&reduced_matrix);
            println!("values:");
            println!("{:?}", reduced_vals);
        }
        println!("solved joltage: {}", joltage_answer);
        part_2_presses += joltage_answer;

        // println!("max cycles: {}", solve_joltage(&machine) - 1);
    }

    println!("Total button presses: {}", total_presses);
    println!("Total part 2 presses: {}", part_2_presses);
    println!("Ran in {}ms", start.elapsed().as_millis());
}

// max number of times for pushing each button
// pushing the button more than this many times causes an overflow
fn max_button_pushes (machine: &Machine) -> Vec<i32> {
// fn max_button_pushes (buttons: &Vec<HashSet<usize>>, joltage_requirements: &Vec<i32>) -> Vec<i32> {
    let mut max_pushes = vec![i32::MAX; machine.buttons.len()];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for button_val in button {
            if (machine.joltage_requirements[*button_val] as i32) < max_pushes[button_idx] {
                max_pushes[button_idx] = machine.joltage_requirements[*button_val] as i32;
            }
        }
    }

    return max_pushes;
}

fn parse_input () -> Vec<Machine> {
    let mut machines = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let machine = Machine::from_input(line);
        machines.push(machine);
    }

    return machines;
}

fn check_joltage_solution(machine: &Machine, button_pushes: &Vec<i32>) -> bool {
    let mut totals = vec![0; machine.joltage_requirements.len()];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for button_val in button {
            totals[*button_val] += button_pushes[button_idx];
        }
    }

    let mut solved = true;
    for (idx, val) in machine.joltage_requirements.iter().enumerate() {
        if totals[idx] != *val as i32 {
            solved = false
        }
    }

    if !solved {
        println!("requirements:");
        println!("{:?}", machine.joltage_requirements);
        println!("my answer:");
        println!("{:?}", totals);
    }

    return solved
}

fn solve_joltage (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, max_pushes: &Vec<i32>, max_total: i32) -> Vec<i32> {
    let mut best_solution = i32::MAX;
    let mut best_solution_vars = vec![];

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut target_row = None;
    let mut max_vars: i32 = 0;

    let mut improved_max_pushes = max_pushes.clone();

    for row_idx in 0..n_rows {
        let var_count = matrix[row_idx].iter().map(|v| v.abs().signum()).sum();
        if var_count > max_vars {
            max_vars = var_count;
            target_row = Some(row_idx)
        }

        for row_idx in 0..n_rows {
            let mut all_positive = true;
            let mut all_negative = true;
            for col_idx in 0..n_cols {
                if matrix[row_idx][col_idx] > 0 {
                    all_negative = false
                }

                if matrix[row_idx][col_idx] < 0 {
                    all_positive = false
                }
            }

            // if all_positive {
            for col_idx in 0..n_cols {
                if matrix[row_idx][col_idx] != 0 && (all_positive || all_negative) {
                    // improved_max_pushes[col_idx] = min(improved_max_pushes[col_idx], values[row_idx]).abs();
                }
            }
            // }
        }
    }

    if target_row.is_none() {
        panic!("no row to target")
    }
    let target_row_idx = target_row.unwrap();
    println!("targeting row {}", target_row_idx);

    let n_vars = max_vars as usize - 1;

    if values[target_row_idx].signum() == -1 {
        values[target_row_idx] *= -1;
        for col_idx in 0..n_cols {
            matrix[target_row_idx][col_idx] *= -1;
        }
    }

    // let max_val = max(original_vals[target_row_idx], values[target_row_idx]);
    let target_val = values[target_row_idx];
    let mut max_vals = vec![];

    let mut var_indices = vec![];
    let mut coefficients = vec![];

    for col_idx in 0..n_cols {
        if matrix[target_row_idx][col_idx] != 0 {
            var_indices.push(col_idx);
            coefficients.push(matrix[target_row_idx][col_idx]);

            
            max_vals.push(improved_max_pushes[col_idx]);
        }
    }

    println!("Trying buttons: {:?}", var_indices);
    println!("Max pushes of each button: {:?}", max_vals);

    let slack_var_idx = var_indices.pop().unwrap();
    let slack_coefficient = coefficients.pop().unwrap();

    if VERBOSE {
        println!("slack var idx: {}", slack_var_idx);
        println!("slack coefficient: {}", slack_coefficient);
    }

    let mut vars_optional = Some(vec![0; n_vars]);
    let mut n_var_combinations = 0;

    let mut var_combinations = HashSet::new();

    loop {
        n_var_combinations += 1;
        if vars_optional.is_none() {
            break;
        }
        let vars = vars_optional.unwrap();
        if var_combinations.contains(&vars){
            println!("Seen vars {:?} before", vars);
            panic!();
        }
        var_combinations.insert(vars.clone());

        let mut verbose_override = false;
        // if vars[0] == 11 && vars[1] == 5 {
        //     verbose_override = true
        // }
        // println!("vars: {:?}", vars);

        let mut var_array = vec![0; n_cols];
        let mut var_mask = vec![false; n_cols];

        for (idx, var_idx) in var_indices.iter().enumerate() {
            var_array[*var_idx] = vars[idx];
            var_mask[*var_idx] = true;
        }
        if VERBOSE || verbose_override { println!("var array: {:?}", var_array); }
        if VERBOSE || verbose_override { println!("var mask: {:?}", var_mask); }

        // attempt solution
        let mut sum = 0;
        for var_idx in 0..n_vars {
            sum += vars[var_idx] * coefficients[var_idx];
        }
        if VERBOSE || verbose_override {
            println!("sum: {}", sum);
            println!("target val: {}", target_val);
        }

        if target_val != 0 && (target_val - sum) != 0 && ((target_val - sum).abs() < slack_coefficient.abs() || (target_val - sum).abs() % slack_coefficient.abs() != 0 || (target_val - sum).signum() != slack_coefficient.signum()) {
            // not a solution
            vars_optional = next_vars(&vars, &max_vals, max_total);
            continue;
        }

        let slack_var = (target_val - sum) / slack_coefficient;
        if VERBOSE || verbose_override { println!("slack var: {}", slack_var); }

        var_array[slack_var_idx] = slack_var;
        var_mask[slack_var_idx] = true;

        if var_array.iter().sum::<i32>() > best_solution {
            // won't be the best solution, don't bother solving
            vars_optional = next_vars(&vars, &max_vals, max_total);
            continue;
        }

        // attempt to solve
        let mut solvable = true;
        let mut possible_best_solution = true;
        let mut solved_rows = vec![false; n_rows];
        solved_rows[target_row_idx] = true;
        let mut skip = false;
        let mut new_vars = vars.clone();

        loop {
            let mut solved_row_count = 0;
            let mut solvable_row_count = 0;

            for row_idx in 0..n_rows {
                if !solved_rows[row_idx] && is_row_solvable(&matrix, &var_mask, row_idx) {
                    if VERBOSE || verbose_override { println!("attempting to solve row: {}", row_idx); }
                    solvable_row_count += 1;
                    let row_solvable = solve_for_row(matrix, values, &mut var_array, &mut var_mask, row_idx, verbose_override);
                    if !row_solvable {
                        // no way to solve given vars
                        if VERBOSE || verbose_override { println!("failed to solve row: {}", row_idx); }
                        solvable = false;
                        break
                    }

                    if VERBOSE || verbose_override {
                        println!("solved row");
                        println!("new vars: {:?}", var_array);
                        println!("new var mask: {:?}", var_mask );
                    }

                    solved_rows[row_idx] = true;
                    solved_row_count += 1;

                    if var_array.iter().sum::<i32>() > best_solution {
                        // won't be the best solution, don't bother solving
                        if VERBOSE || verbose_override  { println!("no longer the best solution"); }
                        possible_best_solution = false;
                        break;
                    }
                }
            }
            
            if solvable_row_count == 0 && !var_mask.iter().all(|m| *m) {
                let mut unsolved_var_idx = 0;
                for row_idx in 0..n_rows {
                    if !var_mask[row_idx] {
                        unsolved_var_idx = row_idx;
                        break;
                    }
                }
                if !var_indices.contains(&unsolved_var_idx) {
                    // println!("current vars")
                    println!("Attempting to solve with additional variable: {} | max val {}", unsolved_var_idx, improved_max_pushes[unsolved_var_idx]);
                    max_vals.push(improved_max_pushes[unsolved_var_idx]);
                    var_indices.push(unsolved_var_idx);

                    new_vars.push(0);
                    skip = true;

                    break;
                }
            }

            if solved_rows.iter().all(|m| *m) {
                break;
            }

            // can't solve, won't be the best solution, or there are no solvable rows
            if !solvable || !possible_best_solution || solvable_row_count == 0 {
                solvable = false;
                break;
            }
        }

        if var_mask.iter().all(|m| *m) && solvable {
            if VERBOSE || verbose_override {
                println!("Possible solution:");
                println!("{:?}", var_array);
            }
            // panic!();
            if var_array.iter().sum::<i32>() < best_solution {
                best_solution = var_array.iter().sum::<i32>();
                best_solution_vars = var_array;
            }
            vars_optional = next_vars(&vars, &max_vals, max_total);
            continue;
        }

        if skip {
            vars_optional = next_vars(&new_vars, &max_vals, max_total);
            continue;
        }

        if !solvable {
            // if vars[0] == 24 && vars[1] == 2 && vars[2] == 7 {
            //     println!("var array: {:?}", var_array);
            //     println!("var mask: {:?}", var_mask);

            //     panic!()
            // }
            vars_optional = next_vars(&vars, &max_vals, max_total);
            continue;
        }

        // if var_array.iter().sum::<i32>() < best_solution {
        //     if VERBOSE {
        //         println!("Possible solution:");
        //         println!("{:?}", var_array);
        //     }
        //     best_solution = var_array.iter().sum::<i32>();
        //     best_solution_vars = var_array;
        // }

        // if vars[0] == 24 && vars[1] == 2 && vars[2] == 7 {
        //     panic!()
        // }

        vars_optional = next_vars(&vars, &max_vals, max_total);
    }


    println!("Best solution: {}", best_solution);
    println!("Best vars: {:?}", best_solution_vars);
    println!("Tried {} var combinations", n_var_combinations);
    return best_solution_vars;
}

fn is_row_solvable (matrix: &Vec<Vec<i32>>, var_mask: &Vec<bool>, row_idx: usize) -> bool {
    let n_cols = matrix[row_idx].len();
    let mut n_slack_vars = 0;

    for col_idx in 0..n_cols {
        if !var_mask[col_idx] && matrix[row_idx][col_idx] != 0 {
            n_slack_vars += 1;
        }
    }

    return n_slack_vars <= 1;
}

fn solve_for_row (matrix: &Vec<Vec<i32>>, values: &Vec<i32>, var_array: &mut Vec<i32>, var_mask: &mut Vec<bool>, row_idx: usize, verbose_override: bool) -> bool {
    let n_cols = matrix[row_idx].len();
    let target_val = values[row_idx];

    
    if VERBOSE || verbose_override {
        println!("Solving for row: {}", row_idx);
        println!("Matrix:");
        print_matrix(matrix);
        println!("values:");
        println!("{:?}", values);
        println!("vars:");
        println!("{:?}", var_array);
        println!("Target val: {}", target_val);
    }

    // if target_val == 0 {
    //     for col_idx in 0..n_cols {
    //         if matrix[row_idx][col_idx] != 0 {
    //             var_mask[col_idx] = true;
    //         }
    //     }
    //     return true
    // }

    let mut already_solved = true;
    let mut sum = 0;

    let mut slack_coefficient = 0;
    let mut slack_var_idx = 0;

    for col_idx in 0..n_cols {
        if matrix[row_idx][col_idx] != 0 && !var_mask[col_idx] {
            already_solved = false;
            slack_coefficient = matrix[row_idx][col_idx];
            slack_var_idx = col_idx;
            // break;
        }
        sum += matrix[row_idx][col_idx] * var_array[col_idx];
    }
    if VERBOSE || verbose_override { println!("Already solved: {}", already_solved);  }
    if VERBOSE || verbose_override { println!("Sum: {}", sum);  }

    if already_solved && sum == target_val {
        // row is already solved with other variables
        return true
    } else if already_solved && sum != target_val {
        // solution isn't consistent
        if VERBOSE || verbose_override { println!("inconsistent solution: sum is {}, target is {}", sum, target_val); }
        return false
    }
    // not already solved, need to solve row

    if (target_val - sum) == 0 {
        var_mask[slack_var_idx] = true;
        return true
    }

    let numerator = (target_val - sum);
    // let denominator = slack_coefficient;

    // find slack var
    if numerator.abs() < slack_coefficient.abs() || numerator.abs() % slack_coefficient.abs() != 0 || numerator.signum() != slack_coefficient.signum() {
        if VERBOSE || verbose_override { println!("solution is not valid: numerator is {}, denominator is {}", numerator, slack_coefficient); }
        // unable to solve, does not divide into whole number
        return false
    }

    if VERBOSE || verbose_override { println!("solution found: {}", numerator / slack_coefficient); }

    // solution found
    var_array[slack_var_idx] = numerator / slack_coefficient;
    var_mask[slack_var_idx] = true;

    return true;
}


fn next_vars (vars: &Vec<i32>, max_vals: &Vec<i32>, max_total: i32) -> Option<Vec<i32>> {
    let n_vars = vars.len();
    // if vars.len() == 0 || vars.iter().sum::<i32>() == max_val * n_vars as i32 {
    //     return None
    // }
    if vars.len() == 0 {
        return None
    }

    if vars[n_vars - 1] == max_vals[n_vars - 1] {
        let new_vars = next_vars(&vars[0..(n_vars-1)].to_vec(), &max_vals[0..(n_vars-1)].to_vec(), max_total);
        if new_vars.is_none() {
            return None
        }
        let mut nv = new_vars.unwrap();
        nv.push(0);
        return Some(nv);
    }

    let mut new_vars = vars.clone();
    new_vars[n_vars - 1] += 1;

    // if new_vars.iter().sum::<i32>() > max_total {
    //     return next_vars(&new_vars, max_vals, max_total);
    // }

    return Some(new_vars);
}


fn make_matrix (machine: &Machine) -> (Vec<Vec<i32>>, Vec<i32>) {
    let n_buttons = machine.buttons.len();
    let n_joltage_levels = machine.joltage_requirements.len();

    let mut joltage_buttons = vec![0_u32; n_joltage_levels];
    let mut matrix = vec![vec![0_i32; n_buttons]; n_joltage_levels];
    let mut values = machine.joltage_requirements.iter().map(|v| *v as i32).collect();

    for button_idx in 0..n_buttons {
        for joltage_idx in machine.buttons[button_idx].iter() {
            joltage_buttons[*joltage_idx] += 1;

            matrix[*joltage_idx][button_idx] = 1;
        }
    }

    println!("Max before reducing: {}", joltage_buttons.iter().max().unwrap());
    // reduce_matrix(&mut matrix, &mut values);


    // println!("reduced to:");
    // println!("{:?}", matrix);
    // println!("{:?}", values);

    // let max_incrementers = joltage_buttons.iter().max().unwrap();

    // let mut reduced_joltage_buttons = vec![0_u32; n_joltage_levels];
    // for button_idx in 0..n_buttons {
    //     for joltage_idx in 0..n_joltage_levels {
    //         if matrix[joltage_idx][button_idx] != 0 {
    //             reduced_joltage_buttons[joltage_idx] += 1;
    //         }
    //         matrix[joltage_idx][button_idx] = 1;
    //     }
    // }
    // println!("Max after reducing: {}", reduced_joltage_buttons.iter().max().unwrap());

    return (matrix, values)
}


fn reduce_matrix (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>) {
    return do_reduction(matrix, values, 0);
}


fn do_reduction (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, col_counter: usize) {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    if col_counter == n_cols {
        // reduced on every column
        return;
    }


    if VERBOSE {
        println!("Current matrix:");
        print_matrix(&matrix);
        println!("{:?}", values);
    }

    let mut target_col = None;
    for i in col_counter..n_cols {
        let col_count = count_column(matrix, i);
        if col_count == 0 {
        } else if col_count > 1 {
            target_col = Some(i);
            break;
        }
    }


    if target_col.is_none() {
        return
        // done
    }
    let col_idx = target_col.unwrap();
    if VERBOSE { println!("targeting column {}", col_idx); }

    let pivot_rows = find_source_and_target_row(&matrix, col_idx);
    if pivot_rows.is_none() {
        // not possible to pivot on this column
        // try next col
        return do_reduction(matrix, values, col_counter + 1);
    }
    let (source_row_idx, target_row_idx) = pivot_rows.unwrap();

    // let mut source_row = find_source_row(&matrix, col_idx, &ignore_rows);
    // if source_row.is_none() {
    //     // unable to find source row to reduce on
    //     // try next column
    //     return do_reduction(matrix, values, col_counter + 1, vec![]);
    // }
    // let mut source_row_idx = source_row.unwrap();
    // if VERBOSE { println!("attempting to source from row {}", source_row_idx); }

    // let mut target_row = None;
    // for row_idx in 0..n_rows {
    //     if row_idx == source_row_idx {
    //         continue;
    //     }

    //     if matrix[source_row_idx][col_idx].abs() > matrix[row_idx][col_idx].abs() {
    //         // won't be able to reduce on this row
    //         let mut new_ignores: Vec<usize> = ignore_rows.clone();
    //         new_ignores.push(source_row_idx);
    //         return do_reduction(matrix, values, col_counter, new_ignores);
    //     }

    //     if matrix[row_idx][col_idx] != 0 {
    //         target_row = Some(row_idx);
    //         break
    //     }
    // }
    if VERBOSE { println!("sourcing from row {}", source_row_idx); }

    // if target_row.is_none() {
    //     // unable to find target row to reduce on
    //     // try next column
    //     return do_reduction(matrix, values, col_counter + 1, vec![]);
    // }
    // let target_row_idx = target_row.unwrap();
    if VERBOSE { println!("targeting row {}", target_row_idx); }

    let sign = matrix[target_row_idx][col_idx].signum() * matrix[source_row_idx][col_idx].signum();
    if sign == 0 {
        panic!("zero sign")
    }
    if VERBOSE { println!("sign is {}", sign); }

    // subtract source row from target row
    for col in 0..n_cols {
        matrix[target_row_idx][col] -= matrix[source_row_idx][col] * sign;
    }
    values[target_row_idx] -= values[source_row_idx] * sign;
    scale_row(matrix, values, target_row_idx);

    return do_reduction(matrix, values, col_counter);
}

fn scale_row (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>, row_idx: usize) {
    let n_cols = matrix[row_idx].len();
    let mut gcd = 1;
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
        // let val = matrix[row_idx][col_idx];
        // let sign = val.signum();
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

                if source_row_idx != target_row_idx && !(matrix[source_row_idx][col_idx].abs() > matrix[target_row_idx][col_idx].abs()) {
                    return Some((source_row_idx, target_row_idx));
                }
            }
        }
    }

    return None
}

fn find_source_row (matrix: &Vec<Vec<i32>>, col_idx: usize, ignore_rows: &Vec<usize>) -> Option<usize> {
    let n_rows = matrix.len();

    for row_idx in 0..n_rows {
        if matrix[row_idx][col_idx] == 0 || ignore_rows.contains(&row_idx) || matrix[row_idx][col_idx].abs() > matrix[row_idx][col_idx].abs(){
            continue;
        }

        let mut valid = true;
        for c in 0..col_idx {
            if matrix[row_idx][c] != 0 {
                valid = false;
                break
            }
        }

        if valid {
            return Some(row_idx);
        }
    }

    return None
}

fn count_column (matrix: &Vec<Vec<i32>>, col_idx: usize) -> usize {
    let n_rows = matrix.len();

    let mut count = 0;
    for i in 0..n_rows {
        if matrix[i][col_idx] != 0 {
            count += 1;
        }
    }

    return count
}

fn solve_machine (machine: &Machine) -> u32 {
    let n_buttons = machine.buttons.len();

    let mut min_buttons = u32::MAX;

    println!("machine target: {}",  machine.start_config_binary);
    println!("buttons: {:?}", machine.buttons_binary);

    for mask in 0..POWERS_OF_2[n_buttons] {
        let mut sum = 0;

        for (idx, bin) in machine.buttons_binary.iter().enumerate() {
            // println!("binary: {} | mask: {} | bitwise and: {}", bin, mask, *bin & mask);
            if POWERS_OF_2[idx] & mask > 0 {
                // println!("Pressing button {}", bin);
                sum ^= *bin;
            }
            // sum ^= *bin & mask;
        }

        // println!("mask: {}, target: {}, sum: {}", mask, machine.start_config_binary, sum);

        let button_presses = mask.count_ones();

        // if button_presses != n_buttons_presses {
        //     println!("counted {} button presses, mask is {}, one_count is {}", n_buttons_presses, mask, button_presses);
        //     panic!()
        // }

        if sum == machine.start_config_binary && button_presses < min_buttons {
            min_buttons = button_presses;
        }
    }

    return min_buttons;
}

struct Machine {
    raw: String,
    n_lights: usize,
    start_config: Vec<char>,
    start_config_binary: usize,
    buttons: Vec<HashSet<usize>>,
    buttons_binary: Vec<usize>,
    joltage_requirements: Vec<usize>
}

impl Machine {
    fn from_input (input: &str) -> Machine {
        let sections = input.split_whitespace().collect::<Vec<&str>>();

        let light_section = sections[0];
        let mut lights = vec![];
        let mut buttons = vec![];

        let n_lights = light_section.len() - 2;
        let mut light_binary = 0;

        // get lights
        for (c_idx, c) in light_section.chars().enumerate() {
            if c_idx == 0 && c != '[' {
                panic!("bad char")
            }

            if c_idx == (light_section.len() -1 ) && c != ']' {
                panic!("bad char")
            }

            if c_idx != 0 && c_idx != (light_section.len() -1 ) {
                lights.push(c);

                if c == ON_CHAR {
                    light_binary += 2_usize.pow((c_idx-1) as u32);
                }
            }
        }

        let mut section_idx = 1;

        let mut buttons_binary = vec![];

        loop {
            let possible_button = parse_button(sections[section_idx]);
            if possible_button.is_some() {
                let mut button_binary = 0;

                for button_val in possible_button.clone().unwrap() {
                    button_binary += 2_usize.pow(button_val as u32);
                }

                buttons.push(possible_button.unwrap());
                buttons_binary.push(button_binary);
                section_idx += 1;
            } else {
                break
            }
        }

        let possible_joltage = parse_joltage(sections[section_idx]);

        if possible_joltage.is_none() {
            panic!("unparsable joltage")
        }

        return Machine { n_lights: lights.len(), start_config: lights, start_config_binary: light_binary, buttons: buttons, buttons_binary: buttons_binary, joltage_requirements: possible_joltage.unwrap(), raw: input.to_string() }
    }
}

fn parse_button (button_str: &str) -> Option<HashSet<usize>> {
    let button_len = button_str.len();
    if button_str.chars().nth(0) != Some('(') || button_str.chars().nth(button_len-1) != Some(')') {
        return None
    }

    let mut button = HashSet::new();

    for value in button_str[1..(button_len-1)].split(',') {
        button.insert(value.parse::<usize>().unwrap());
    }

    return Some(button)
}

fn parse_joltage (joltage_str: &str) -> Option<Vec<usize>> {
    let joltage_len = joltage_str.len();
    if joltage_str.chars().nth(0) != Some('{') || joltage_str.chars().nth(joltage_len-1) != Some('}') {
        return None
    }

    let mut joltage = vec![];

    for value in joltage_str[1..(joltage_len-1)].split(',') {
        joltage.push(value.parse::<usize>().unwrap());
    }

    return Some(joltage)
}

// fn ham_weight(x: &[u8]) -> u64 {
//     x.iter().fold(0, |a, b| a + b.count_ones() as u64)
// }

fn print_matrix (matrix: &Vec<Vec<i32>>) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
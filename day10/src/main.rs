use std::cmp::{max, min};
use std::env::var;
use std::i32;
use std::{collections::HashSet, fs, time::Instant};
use std::iter::FromIterator;

const FILENAME: &str = "./test_input.txt";
const ON_CHAR: char = '#';
const POWERS_OF_2: [usize; 14] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];

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

        let (mut matrix, mut values) = make_matrix(&machine);
        let solved_joltage = solve_joltage(&mut matrix, &mut values);

        if solved_joltage == i32::MAX {
            println!("failed to solve machine {}", machine.raw);
            println!("matrix representation:");
            println!("{:?}", matrix);
            println!("values:");
            println!("{:?}", values);
        }
        println!("solved joltage: {}", solved_joltage);
        part_2_presses += solved_joltage;

        // println!("max cycles: {}", solve_joltage(&machine) - 1);
    }

    println!("Total button presses: {}", total_presses);
    println!("Total part 2 presses: {}", part_2_presses);
    println!("Ran in {}ms", start.elapsed().as_millis());
}

fn parse_input () -> Vec<Machine> {
    let mut machines = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let machine = Machine::from_input(line);
        machines.push(machine);
    }

    return machines;
}

fn solve_joltage (matrix: &mut Vec<Vec<i32>>, values: &mut Vec<i32>) -> i32 {
    let mut best_solution = i32::MAX;

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut target_row = None;
    let mut max_vars: i32 = 0;

    for row_idx in 0..n_rows {
        let var_count = matrix[row_idx].iter().map(|v| v.abs().signum()).sum();
        if var_count > max_vars {
            max_vars = var_count;
            target_row = Some(row_idx)
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

    let max_val = values[target_row_idx];
    let target_val = values[target_row_idx];

    let mut var_indices = vec![];
    let mut coefficients = vec![];

    for col_idx in 0..n_cols {
        if matrix[target_row_idx][col_idx] != 0 {
            var_indices.push(col_idx);
            coefficients.push(matrix[target_row_idx][col_idx]);
        }
    }

    let slack_var_idx = var_indices.pop().unwrap();
    let slack_coefficient = coefficients.pop().unwrap();

    println!("slack var idx: {}", slack_var_idx);
    println!("slack coefficient: {}", slack_coefficient);

    let mut vars_optional = Some(vec![0; n_vars]);

    loop {
        if vars_optional.is_none() {
            break;
        }
        let vars = vars_optional.unwrap();
        // println!("vars: {:?}", vars);

        let mut var_array = vec![0; n_cols];
        let mut var_mask = vec![false; n_cols];

        for (idx, var_idx) in var_indices.iter().enumerate() {
            var_array[*var_idx] = vars[idx];
            var_mask[*var_idx] = true;
        }
        println!("var array: {:?}", var_array);

        // attempt solution
        let mut sum = 0;
        for var_idx in 0..n_vars {
            sum += vars[var_idx] * coefficients[var_idx];
        }

        if target_val != 0 && (target_val - sum) != 0 && (target_val - sum).abs() < slack_coefficient.abs() || (target_val - sum).abs() % slack_coefficient.abs() != 0 || (target_val - sum).signum() != slack_coefficient.signum() {
            // not a solution
            vars_optional = next_vars(&vars, max_val);
            continue;
        }

        let slack_var = (target_val - sum) / slack_coefficient;
        println!("slack var: {}", slack_var);

        var_array[slack_var_idx] = slack_var;
        var_mask[slack_var_idx] = true;

        if var_array.iter().sum::<i32>() > best_solution {
            // won't be the best solution, don't bother solving
            vars_optional = next_vars(&vars, max_val);
            continue;
        }

        // attempt to solve
        let mut solvable = true;
        for row_idx in 0..n_rows {
            if row_idx == target_row_idx {
                continue;
            }
            println!("attempting to solve row: {}", row_idx);

            solvable &= solve_for_row(matrix, values, &mut var_array, &mut var_mask, row_idx);
            if !solvable {
                break
            }

            println!("solved row");
            println!("new vars: {:?}", var_array);
            println!("new var mask: {:?}", var_mask );

            if var_array.iter().sum::<i32>() > best_solution {
                // won't be the best solution, don't bother solving
                solvable = false;
                break;
            }
        }

        if !solvable {
            vars_optional = next_vars(&vars, max_val);
            continue;
        }

        if var_array.iter().sum::<i32>() < best_solution {
            println!("Possible solution:");
            println!("{:?}", var_array);
            best_solution = var_array.iter().sum::<i32>();
        }

        vars_optional = next_vars(&vars, max_val);
    }


    return best_solution;
}

fn solve_for_row (matrix: &Vec<Vec<i32>>, values: &Vec<i32>, var_array: &mut Vec<i32>, var_mask: &mut Vec<bool>, row_idx: usize) -> bool {
    let n_cols = matrix[row_idx].len();
    let target_val = values[row_idx];

    if target_val == 0 {
        for col_idx in 0..n_cols {
            if matrix[row_idx][col_idx] != 0 {
                var_mask[col_idx] = true;
            }
        }
        return true
    }

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
    if already_solved && sum == target_val {
        // row is already solved with other variables
        return true
    } else if already_solved && sum != target_val {
        // solution isn't consistent
        return false
    }
    // not already solved, need to solve row

    if (target_val - sum) == 0 {
        var_mask[slack_var_idx] = true;
        return true
    }

    // find slack var
    if (target_val - sum).abs() < slack_coefficient.abs() || (target_val - sum).abs() % slack_coefficient.abs() != 0 || (target_val - sum).signum() != slack_coefficient.signum() {
        // unable to solve, does not divide into whole number
        return false
    }

    // solution found
    var_array[slack_var_idx] = (target_val - sum) / slack_coefficient;
    var_mask[slack_var_idx] = true;

    return true;
}


fn next_vars (vars: &Vec<i32>, max_val: i32) -> Option<Vec<i32>> {
    let n_vars = vars.len();
    // if vars.len() == 0 || vars.iter().sum::<i32>() == max_val * n_vars as i32 {
    //     return None
    // }
    if vars.len() == 0 {
        return None
    }

    if vars[n_vars - 1] == max_val {
        let new_vars = next_vars(&vars[0..(n_vars-1)].to_vec(), max_val);
        if new_vars.is_none() {
            return None
        }
        let mut nv = new_vars.unwrap();
        nv.push(0);
        return Some(nv);
    }

    let mut new_vars = vars.clone();
    new_vars[n_vars - 1] += 1;

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
    reduce_matrix(&mut matrix, &mut values);


    println!("reduced to:");
    println!("{:?}", matrix);
    println!("{:?}", values);

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
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    println!("{:?}", matrix);
    println!("{:?}", values);

    let mut target_col = None;
    for i in 0..n_cols {
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
    println!("targeting column {}", col_idx);

    let source_row = find_source_row(&matrix, col_idx);
    if source_row.is_none() {
        return
    }
    let source_row_idx = source_row.unwrap();
    println!("sourcing from row {}", source_row_idx);

    let mut target_row = None;
    for row_idx in 0..n_rows {
        if row_idx == source_row_idx {
            continue;
        }

        if matrix[source_row_idx][col_idx].abs() > matrix[row_idx][col_idx].abs() {
            continue;
        }

        if matrix[row_idx][col_idx] != 0 {
            target_row = Some(row_idx);
            break
        }
    }

    if target_row.is_none() {
        return
    }
    let target_row_idx = target_row.unwrap();
    println!("targeting row {}", target_row_idx);

    let sign = matrix[target_row_idx][col_idx].signum() * matrix[source_row_idx][col_idx].signum();
    if sign == 0 {
        panic!("zero sign")
    }
    println!("sign is {}", sign);

    // subtract source row from target row
    for col in 0..n_cols {
        matrix[target_row_idx][col] -= matrix[source_row_idx][col] * sign;
    }
    values[target_row_idx] -= values[source_row_idx] * sign;
    scale_row(matrix, values, target_row_idx);

    return reduce_matrix(matrix, values);
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

    // println!("Scaling row {} by factor of {}", row_idx, gcd);

    for col_idx in 0..n_cols {
        // let val = matrix[row_idx][col_idx];
        // let sign = val.signum();
        matrix[row_idx][col_idx] /= gcd;
    }
    values[row_idx] /= gcd;
    // panic!();
    return
}

fn find_source_row (matrix: &Vec<Vec<i32>>, col_idx: usize) -> Option<usize> {
    let n_rows = matrix.len();

    for row_idx in 0..n_rows {
        if matrix[row_idx][col_idx] == 0 {
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
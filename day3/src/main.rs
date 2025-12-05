use std::{fs};
use std::time::{SystemTime};

const FILENAME: &str = "./input.txt";

fn main() {
    let banks = parse_input();

    let mut total = 0;
    let mut total_2 = 0;
    let mut total_2_1 = 0;

    for (bank_idx, bank) in banks.into_iter().enumerate() {
        let mut first_digit = 0;
        let mut first_digit_index = 0;;

        for (i, value) in bank[..(bank.len()-1)].iter().enumerate() {
            if *value > first_digit {
                first_digit = *value;
                first_digit_index = i;
            }
        }

        let second_digit = bank[(first_digit_index+1)..].into_iter().max().unwrap();

        let joltage_2 = turn_on_n_batteries(&bank, 12);
        let joltage_2_1 = vec_to_num(&n_batteries(bank.as_slice(), 12));

        println!("Bank {} | value 1 {}", bank_idx,  first_digit * 10 + second_digit);
        println!("Bank {} | value 2 {}", bank_idx,  joltage_2);
        println!("Bank {} | value 2 {} (recursive way)", bank_idx,  joltage_2_1);

        total += (first_digit * 10 + second_digit);
        total_2 += joltage_2;
        total_2_1 += joltage_2_1;

        println!("{:?}", n_batteries(bank.as_slice(), 12));
    }

    println!("Total 1 is {}", total);
    println!("Total 2 is {}", total_2);
    println!("Total 2 is {} (recursive way)", total_2_1);
}

fn n_batteries (bank: &[u64], n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }
    let mut first_digit = 0;
    let mut first_digit_index = 0;;

    for (i, value) in bank[..(bank.len() - n as usize + 1)].iter().enumerate() {
        if *value > first_digit {
            first_digit = *value;
            first_digit_index = i;
        }
    }

    let mut result = vec![first_digit];
    result.append(&mut n_batteries(&bank[(first_digit_index + 1)..], n - 1));

    return result
}

fn turn_on_n_batteries (bank: &Vec<u64>, n: u64) -> u64 {
    if bank.len() as u64 <= n {
        println!("{:?}", bank);
        return vec_to_num(bank);
    }

    let mut digits = vec![];
    let mut digit_indices = vec![];

    for digit_idx in 0..(n-1) {
        let mut max = 0;
        let mut max_idx = 0;

        let start_idx = if digit_idx == 0 { 0 } else { digit_indices[digit_idx as usize - 1] + 1 };

        for (i, value) in bank[start_idx..(bank.len() - n as usize + digit_idx as usize + 1)].iter().enumerate() {
            if *value > max {
                max = *value;
                max_idx = i;
            }
        }
        digits.push(max);
        digit_indices.push(max_idx + start_idx);
    }

    // println!("{:?}", digits);

    let last_digit = bank[(digit_indices[n as usize - 2]+1)..].into_iter().max().unwrap();
    digits.push(*last_digit);

    return vec_to_num(&digits);
}

fn vec_to_num (bank: &Vec<u64>) -> u64 {
    let mut total = 0;
    let bank_len = bank.len();

    for (i, value) in bank.iter().enumerate() {
        // total += ((bank_len - i - 1) as u64).pow(10) * value;
        total += 10_u64.pow((bank_len - i - 1) as u32) * value;
    }
    
    return total
}

fn parse_input () -> Vec<Vec<u64>> {
    let mut banks = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        banks.push(
            line.chars().map(|c| u64::from(c.to_digit(10).unwrap())).collect::<Vec<u64>>()
        );
    }
    return banks;
}
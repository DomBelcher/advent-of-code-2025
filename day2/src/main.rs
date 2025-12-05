use std::{fs};
use std::time::{SystemTime};

const FILENAME: &str = "./input.txt";

fn main() {
    let start = SystemTime::now();
    let ranges = parse_input();

    println!("{:?}", ranges);

    let mut count = 0;
    let mut total_1 = 0;
    let mut total_2 = 0;

    for range in ranges {
        let start = range.0;
        let end = range.1;

        for num in start..(end + 1) {
            if is_reduplicated(num) {
                count += 1;
                total_1 += num;
            }

            if is_repeated(num) {
                total_2 += num;
            }
        }
    }

    println!("Total 1 is: {}", total_1);
    println!("Total 2 is: {}", total_2);

    let end = SystemTime::now();
    println!("Program ran in {}ms", end.duration_since(start).unwrap().subsec_millis());


}

fn is_repeated (num: i64) -> bool {
    let num_string = num.to_string();
    let num_len = num_string.len();

    let mut is_repeated = false;

    for i in 2..(num_len + 1) {
        if num_len % i == 0 {
            if is_repeated_n_times(num, i) {
                return true
            }
        }
    }

    return is_repeated
}

fn is_repeated_n_times(num: i64, n: usize) -> bool {
    let num_string = num.to_string();
    let num_len = num_string.len();

    let step_size = num_len/n;
    
    let mut s1 = &num_string[..step_size];

    for i in 1..n {
        let s2 = &num_string[(step_size * i)..(step_size * (i + 1))];

        if s1 != s2 {
            return false
        }
        s1 = s2;
    }

    return true
}

fn is_reduplicated (num: i64) -> bool {
    let num_string = num.to_string();
    let num_len = num_string.len();

    if num_len % 2 == 1 {
        return false;
    }

    let first_half = &num_string[..num_len/2];
    let second_half = &num_string[num_len/2..];

    return first_half == second_half;
}

fn parse_input () -> Vec<(i64, i64)> {
    let mut ranges = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let mut _ranges = line.split(',').map(|s| s.split('-')).map(|split| {
            let limits = split.map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
            return (*limits.get(0).unwrap(), *limits.get(1).unwrap())
        }).collect::<Vec<(i64, i64)>>();

        ranges.append(&mut _ranges);
    }
    return ranges;
}
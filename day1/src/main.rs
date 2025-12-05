use std::{fs};
use std::time::{SystemTime};

const FILENAME: &str = "./input.txt";
const START: i32 = 50;

fn main() {
    let start = SystemTime::now();
    let instructions = parse_input();

    println!("{}", (-100000 % 100));

    let password_1 = first_password(&instructions);
    let password_2 = second_password(&instructions);
    
    println!("The first password is: {}", password_1);
    println!("The second password is: {}", password_2);

    let end = SystemTime::now();
    println!("Program ran in {}ms", end.duration_since(start).unwrap().subsec_millis());
}

fn first_password (instructions: &Vec<i32>) -> i32 {
    let mut position = START;
    let mut password = 0;

    for i in instructions {
        position += i;
        position = (position + 100) % 100;
        // println!("{}", position);

        if position == 0 {
            password += 1;
        }
    }
    
    return password
}

fn second_password (instructions: &Vec<i32>) -> i32 {
    let mut password = 0;

    let mut position_before = START;

    for i in instructions {
        let position_after = position_before + i;
        let clicks ;

        if position_before != 0 && position_after == 0 {
            clicks = 1;
        } else if position_before > 0 && position_after < 0 {
            clicks = (position_after * -1) / 100 + 1;
        } else if position_after >= 100 {
            clicks = position_after / 100;
        } else if position_after < -100 {
            clicks = (position_after * -1) / 100;
        } else {
            clicks = 0;
        }
        password += clicks;


        // println!("before: {} | move: {} | after: {} | clicks: {}", position_before, i, position_after, clicks);
    
        position_before = position_after.rem_euclid(100);
    }

    return password;
}

fn parse_input () -> Vec<i32> {
    let mut instructions = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let direction_char = line.chars().next();
        let distance = line[1..].parse::<i32>().unwrap();

        match direction_char {
            Some('R') => {
                instructions.push(distance);
            },
            Some('L') => {
                instructions.push(-1 * distance);
            },
            Some(_) => panic!("oh no"),
            None => panic!("oh no"),
        }
    }

    return instructions
}
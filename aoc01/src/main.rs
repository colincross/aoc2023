use std::env;
use std::fs::read_to_string;

const SPELLED_DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn first_digit(s: &String) -> u32 {
    let mut digit = 0;
    let mut digit_location = i32::MAX;

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            digit = c.to_digit(10).unwrap();
            digit_location = i as i32;
            break;
        }
    }

    for (i, &spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
        let location = s.find(spelled_digit).map_or(i32::MAX, |v| v as i32);
        if location < digit_location {
            digit = i as u32;
            digit_location = location;
        }
    }

    if digit_location == i32::MAX {
        panic!("no digit");
    }

    return digit;
}

fn last_digit(s: &String) -> u32 {
    let mut digit = 0;
    let mut digit_location: i32 = -1;
    
    for (i, c) in s.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            digit = c.to_digit(10).unwrap();
            digit_location = (s.len() - i - 1) as i32;
            break;
        }
    }

    for (i, &spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
        let location = s.rfind(spelled_digit).map_or(-1, |v| v as i32);
        if location > digit_location {
            digit = i as u32;
            digit_location = location;
        }
    }

    if digit_location == -1 {
        panic!("no digit");
    }

    return digit;
}

fn first_and_last_digits(s: String) -> u32 {
    let num = first_digit(&s)*10 + last_digit(&s);
    println!("{}: {}", s, num);
    return num;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}",
        read_to_string(&args[1])
             .unwrap()
             .lines()
             .map(String::from)
             .map(first_and_last_digits)
             .sum::<u32>()
            );
}

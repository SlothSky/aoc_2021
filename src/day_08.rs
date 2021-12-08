use ansi_term::Color::{Red, RGB};
use std::{collections::HashMap, fs};

pub fn day_08_main() {
    println!(
        "\n{}\n\t• Appearances of 1, 4, 7, 8: {}\n\t• Sum of all outcomes: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 8:"),
        Red.paint(first_part_08().to_string()),
        Red.paint(second_part_08().to_string())
    )
}

fn first_part_08() -> i32 {
    let digit_string = fs::read_to_string("assets/08/seven_segment.txt").unwrap();
    let mut digit_vec = Vec::new();

    for digits in digit_string.lines() {
        for (index, digit) in digits.split(" | ").into_iter().enumerate() {
            if index == 1 {
                for signal in digit.split_whitespace() {
                    if matches!(signal.len(), 2 | 3 | 4 | 7) {
                        digit_vec.push(signal);
                    }
                }
            }
        }
    }

    digit_vec.len() as i32
}

fn second_part_08() -> i32 {
    let digit_string = fs::read_to_string("assets/08/seven_segment.txt").unwrap();
    let mut digit_vec = Vec::new();

    for digits in digit_string.lines() {
        let test: Vec<&str> = digits.split_terminator(" | ").collect();
        let mut result = String::new();
        let temp = &search_numbers(test[0].trim());

        for signal in test[1].split_whitespace() {
            let signal = signal.trim();
            for (index, (key, value)) in temp.into_iter().enumerate() {
                let mut counter = 0;
                for ch in signal.chars() {
                    if value.contains(ch) {
                        counter += 1;
                    }
                }

                if counter == value.len() && value.len() == signal.len() {
                    result.push_str(&(key.to_string()));
                }

                if result.len() == 4 && index == (temp.len() - 1) {
                    digit_vec.push(result.trim().parse::<i32>().unwrap());
                }
            }
        }
    }

    let mut result_counter = 0;
    for digit in digit_vec {
        result_counter += digit;
    }

    result_counter
}

fn search_numbers(digit_input: &str) -> HashMap<i32, &str> {
    let mut result = HashMap::new();

    while result.len() < 10 {
        for digit in digit_input.split_whitespace() {
            match digit.len() {
                2 => {
                    result.entry(1).or_insert(digit);
                }
                3 => {
                    result.entry(7).or_insert(digit);
                }
                4 => {
                    result.entry(4).or_insert(digit);
                }
                7 => {
                    result.entry(8).or_insert(digit);
                }
                6 => {
                    let mut counter = 0;
                    for ch in digit.chars() {
                        match result.get(&7).or(Some(&"zzz")).unwrap().rfind(ch) {
                            Some(_) => counter += 1,
                            None => (),
                        }
                    }

                    if counter == 2 {
                        result.entry(6).or_insert(digit);
                    } else {
                        counter = 0;
                        for ch in digit.chars() {
                            match result.get(&4).or(Some(&"zzz")).unwrap().rfind(ch) {
                                Some(_) => counter += 1,
                                None => (),
                            }
                        }

                        if counter == 4 {
                            *result.entry(9).or_insert(digit) = digit;
                        } else {
                            *result.entry(0).or_insert(digit) = digit;
                        }
                    }
                }
                5 => {
                    let mut counter = 0;
                    for ch in digit.chars() {
                        match result.get(&1).or(Some(&"zzz")).unwrap().rfind(ch) {
                            Some(_) => counter += 1,
                            None => (),
                        }
                    }

                    if counter == 2 {
                        result.entry(3).or_insert(digit);
                    } else {
                        counter = 0;
                        for ch in digit.chars() {
                            match result.get(&9).or(Some(&"zzz")).unwrap().rfind(ch) {
                                Some(_) => counter += 1,
                                None => (),
                            }
                        }

                        if counter == 4 {
                            *result.entry(2).or_insert(digit) = digit;
                        } else {
                            *result.entry(5).or_insert(digit) = digit;
                        }
                    }
                }
                _ => println!("Unknown pattern occured"),
            }
        }
    }

    result
}

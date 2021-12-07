use ansi_term::Color::{Red, RGB};
use std::{collections::HashMap, fs, ops::Add};

pub fn day_07_main() {
    println!(
        "\n{}\n\t• 1 Fuel required by crabs: {}\n\t• n Fuel required by crabs: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 7:"),
        Red.paint(first_part_07().to_string()),
        Red.paint(second_part_07().to_string())
    )
}

fn first_part_07() -> i32 {
    let crab_string = fs::read_to_string("assets/07/crab_position_list.txt").unwrap();
    let mut crab_vec = Vec::new();

    for crab in crab_string.split_terminator(',') {
        crab_vec.push(crab.trim().parse::<i32>().unwrap());
    }
    crab_vec.sort();

    let mut lowest = i32::MAX;

    for crab in &crab_vec {
        let mut temp = 0;

        for others in &crab_vec {
            temp += i32::abs(others - crab);
        }
        if temp < lowest { lowest = temp; };
    }

    lowest
}

fn second_part_07() -> i64 {
    let crab_string = fs::read_to_string("assets/07/crab_position_list.txt").unwrap();
    let mut crab_vec = Vec::new();

    for crab in crab_string.split_terminator(',') {
        crab_vec.push(crab.trim().parse::<i64>().unwrap());
    }
    crab_vec.sort();

    let mut lowest = i64::MAX;

    for others in *crab_vec.get(0).unwrap()..*crab_vec.get(crab_vec.len() - 1).unwrap() {
        let mut temp = 0;

        for crab in &crab_vec {
            let dif = i64::abs(others - crab);
            temp += (dif * (dif + 1)) / 2;
        }
        if temp < lowest { lowest = temp; };
    }

    lowest
}

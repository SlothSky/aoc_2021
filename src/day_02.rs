use ansi_term::Color::{Red, RGB};
// use std::cmp;
use std::fs;
// use std::io::Read;

struct Coordinates {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

pub fn day_02_main() {
    println!(
        "\n{}\n\t• final position: {}\n\t• final position: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 2:"),
        Red.paint(first_part_02().to_string()),
        Red.paint(second_part_02().to_string())
    )
}

fn first_part_02() -> i32 {
    let course = fs::read_to_string("assets/02/course.txt");

    let mut coordinates = Coordinates {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    for line in course.unwrap().lines() {
        let mut split_line = line.split_whitespace();
        match split_line.next().unwrap() {
            "forward" => {
                coordinates.horizontal_position +=
                    split_line.next().unwrap().parse::<i32>().unwrap()
            }
            "up" => coordinates.depth -= split_line.next().unwrap().parse::<i32>().unwrap(),
            "down" => coordinates.depth += split_line.next().unwrap().parse::<i32>().unwrap(),
            _ => println!("There went something wrong with the input data."),
        }
    }

    coordinates.horizontal_position * coordinates.depth
}

fn second_part_02() -> i32 {
    let course = fs::read_to_string("assets/02/course.txt");

    let mut coordinates = Coordinates {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    for line in course.unwrap().lines() {
        let mut split_line = line.split_whitespace();
        match split_line.next().unwrap() {
            "forward" => {
                let factor = split_line.next().unwrap().parse::<i32>().unwrap();
                coordinates.horizontal_position += factor;
                coordinates.depth += factor * coordinates.aim;
            }
            "up" => coordinates.aim -= split_line.next().unwrap().parse::<i32>().unwrap(),
            "down" => coordinates.aim += split_line.next().unwrap().parse::<i32>().unwrap(),
            _ => println!("There went something wrong with the input data."),
        }
    }

    coordinates.horizontal_position * coordinates.depth
}

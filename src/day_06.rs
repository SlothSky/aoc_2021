use ansi_term::Color::{Red, RGB};
use std::fs;

#[derive(Debug)]
struct Fish {
    timer: i32,
}

pub fn day_06_main() {
    println!(
        "\n{}\n\t• Amount of fish after 80 days: {}\n\t• Amount of fish after 256 days: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 6:"),
        Red.paint(String::from("395627")),
        Red.paint(second_part_06().to_string())
    )
}

fn _first_part_06() -> i32 {
    let init_fish_string = fs::read_to_string("assets/06/init_fish_list.txt").unwrap();
    let mut fish_list = Vec::new();

    for fish_string in init_fish_string.split_terminator(',') {
        let fish = Fish {
            timer: fish_string.parse::<i32>().unwrap(),
        };
        fish_list.push(fish);
    }

    for _day in 0..256 {
        let mut addition_list = Vec::new();

        for fish in &mut fish_list {
            match fish.timer {
                0 => {
                    fish.timer = 6;
                    let child_fish = Fish { timer: 8 };
                    addition_list.push(child_fish);
                }
                _ => fish.timer -= 1,
            }
        }

        for child_fish in addition_list {
            fish_list.push(child_fish);
        }
    }

    let mut fish_counter = 0;
    for _fish in fish_list {
        fish_counter += 1;
    }

    fish_counter
}

fn second_part_06() -> i64 {
    let init_fish_string = fs::read_to_string("assets/06/init_fish_list.txt").unwrap();
    let mut fish_list = [0; 9];

    for fish_string in init_fish_string.split_terminator(',') {
        fish_list[fish_string.parse::<i64>().unwrap() as usize] += 1;
    }

    for _day in 0..256 {
        fish_list.rotate_left(1);
        fish_list[6] += fish_list[8];
    }

    let mut fish_counter = 0;
    for fishes in fish_list {
        fish_counter += fishes;
    }

    fish_counter
}

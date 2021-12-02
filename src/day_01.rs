use ansi_term::Color::{Red, RGB};
use std::cmp;
use std::fs::File;
use std::io::Read;

pub fn day_01_main() {
    println!(
        "{}\n\t• {} increases\n\t• {} increases",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 1:"),
        Red.paint(first_part_01().to_string()),
        Red.paint(second_part_01().to_string())
    )
}

fn first_part_01() -> i32 {
    let file = File::open("assets/01/sonar_sweep_01_01.txt");
    let mut file = file.expect("Something went wrong reading the input file");

    let mut sonar_sweep_list = String::new();
    file.read_to_string(&mut sonar_sweep_list)
        .expect("Something went wrong writing input to vec");

    let mut counter = 0;
    let mut old_value = 0;

    for (i, single_value) in sonar_sweep_list.split('\n').enumerate() {
        let single_value = single_value.trim().parse::<i32>().unwrap();
        match i {
            0 => old_value = single_value,
            _ => {
                if single_value.cmp(&old_value) == cmp::Ordering::Greater {
                    counter += 1
                };
                old_value = single_value;
            }
        }
    }

    counter
}

fn second_part_01() -> i32 {
    let file = File::open("assets/01/sonar_sweep_01_01.txt");
    let mut file = file.expect("Something went wrong reading the input file");

    let mut sonar_sweep_string = String::new();
    let mut sonar_sweep_list = Vec::new();
    file.read_to_string(&mut sonar_sweep_string)
        .expect("Something went wrong writing input to vec");

    for single_value in sonar_sweep_string.split('\n') {
        let single_value = single_value.trim().parse::<i32>().unwrap();
        sonar_sweep_list.push(single_value);
    }

    let mut counter = 0;
    let mut old_three_meas = 0;
    let _limit = sonar_sweep_list.len() - 3;

    for (i, _value) in sonar_sweep_list.iter().enumerate() {
        match i {
            0 => {
                old_three_meas =
                    sonar_sweep_list[i] + sonar_sweep_list[i + 1] + sonar_sweep_list[i + 2];
            }
            1998 => {
                break;
            }
            _ => {
                let current_three_meas =
                    sonar_sweep_list[i] + sonar_sweep_list[i + 1] + sonar_sweep_list[i + 2];
                if current_three_meas.cmp(&old_three_meas) == cmp::Ordering::Greater {
                    counter += 1
                };
                old_three_meas = current_three_meas;
            }
        }
    }

    counter
}

use ansi_term::Color::{Red, RGB};
use std::{collections::HashMap, fs};

pub fn day_05_main() {
    println!(
        "\n{}\n\t• Horizontal and vertical overlaps: {}\n\t• Horizontal, vertical & diagonal overlaps: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 5:"),
        Red.paint(first_part_05().to_string()),
        Red.paint(second_part_05().to_string())
    )
}

fn first_part_05() -> i32 {
    let coordinate_string = fs::read_to_string("assets/05/heat_coordinates.txt").unwrap();
    let mut coordinate_pairs = Vec::new();

    for line in coordinate_string.lines() {
        let mut row_coordinates = ((-1, -1), (-1, -1));
        for (index, pair) in line.split_terminator(" -> ").enumerate() {
            let pair = pair.split_terminator(',');

            for (coordinate_index, coordinate) in pair.enumerate() {
                match index {
                    0 => match coordinate_index {
                        0 => row_coordinates.0 .0 = coordinate.parse::<i32>().unwrap(),
                        1 => row_coordinates.0 .1 = coordinate.parse::<i32>().unwrap(),
                        _ => println!("Something went wrong reading the coordinates"),
                    },
                    1 => match coordinate_index {
                        0 => row_coordinates.1 .0 = coordinate.parse::<i32>().unwrap(),
                        1 => row_coordinates.1 .1 = coordinate.parse::<i32>().unwrap(),
                        _ => println!("Something went wrong reading the coordinates"),
                    },
                    _ => println!("Something went wrong reading the coordinates"),
                }
            }
        }
        coordinate_pairs.push(row_coordinates);
    }

    let mut heat_map = HashMap::new();

    for coordinates in coordinate_pairs {
        if coordinates.0 .1 == coordinates.1 .1 {
            // horizontal row
            if coordinates.0 .0 > coordinates.1 .0 {
                let mut counter = coordinates.0 .0;
                while counter >= coordinates.1 .0 {
                    let list_point = heat_map.entry((counter, coordinates.0 .1)).or_insert(0);
                    *list_point += 1;
                    counter -= 1;
                }
            } else {
                let mut counter = coordinates.0 .0;
                while counter <= coordinates.1 .0 {
                    let list_point = heat_map.entry((counter, coordinates.0 .1)).or_insert(0);
                    *list_point += 1;
                    counter += 1;
                }
            }
        } else if coordinates.0 .0 == coordinates.1 .0 {
            // vertical row
            if coordinates.0 .1 > coordinates.1 .1 {
                let mut counter = coordinates.0 .1;
                while counter >= coordinates.1 .1 {
                    let list_point = heat_map.entry((coordinates.0 .0, counter)).or_insert(0);
                    *list_point += 1;
                    counter -= 1;
                }
            } else {
                let mut counter = coordinates.0 .1;
                while counter <= coordinates.1 .1 {
                    let list_point = heat_map.entry((coordinates.0 .0, counter)).or_insert(0);
                    *list_point += 1;
                    counter += 1;
                }
            }
        }
    }

    let mut result = 0;
    for (_index, value) in heat_map {
        if value > 1 {
            //println!("{:?}", _index);
            result += 1;
        }
    }

    result
}

fn second_part_05() -> i32 {
    let coordinate_string = fs::read_to_string("assets/05/heat_coordinates.txt").unwrap();
    let mut coordinate_pairs = Vec::new();

    for line in coordinate_string.lines() {
        let mut row_coordinates = ((-1, -1), (-1, -1));
        for (index, pair) in line.split_terminator(" -> ").enumerate() {
            let pair = pair.split_terminator(',');

            for (coordinate_index, coordinate) in pair.enumerate() {
                match index {
                    0 => match coordinate_index {
                        0 => row_coordinates.0 .0 = coordinate.parse::<i32>().unwrap(),
                        1 => row_coordinates.0 .1 = coordinate.parse::<i32>().unwrap(),
                        _ => println!("Something went wrong reading the coordinates"),
                    },
                    1 => match coordinate_index {
                        0 => row_coordinates.1 .0 = coordinate.parse::<i32>().unwrap(),
                        1 => row_coordinates.1 .1 = coordinate.parse::<i32>().unwrap(),
                        _ => println!("Something went wrong reading the coordinates"),
                    },
                    _ => println!("Something went wrong reading the coordinates"),
                }
            }
        }
        coordinate_pairs.push(row_coordinates);
    }

    let mut heat_map = HashMap::new();

    for coordinates in coordinate_pairs {
        if coordinates.0 .1 == coordinates.1 .1 {
            // horizontal row
            if coordinates.0 .0 > coordinates.1 .0 {
                let mut counter = coordinates.0 .0;
                while counter >= coordinates.1 .0 {
                    let list_point = heat_map.entry((counter, coordinates.0 .1)).or_insert(0);
                    *list_point += 1;
                    counter -= 1;
                }
            } else {
                let mut counter = coordinates.0 .0;
                while counter <= coordinates.1 .0 {
                    let list_point = heat_map.entry((counter, coordinates.0 .1)).or_insert(0);
                    *list_point += 1;
                    counter += 1;
                }
            }
        } else if coordinates.0 .0 == coordinates.1 .0 {
            // vertical row
            if coordinates.0 .1 > coordinates.1 .1 {
                let mut counter = coordinates.0 .1;
                while counter >= coordinates.1 .1 {
                    let list_point = heat_map.entry((coordinates.0 .0, counter)).or_insert(0);
                    *list_point += 1;
                    counter -= 1;
                }
            } else {
                let mut counter = coordinates.0 .1;
                while counter <= coordinates.1 .1 {
                    let list_point = heat_map.entry((coordinates.0 .0, counter)).or_insert(0);
                    *list_point += 1;
                    counter += 1;
                }
            }
        } else if i32::abs(coordinates.0 .0 - coordinates.1 .0)
            == i32::abs(coordinates.0 .1 - coordinates.1 .1)
        {
            if coordinates.0 .1 > coordinates.1 .1 {
                if coordinates.0 .0 > coordinates.1 .0 {
                    let mut counter_0 = coordinates.0 .0;
                    let mut counter_1 = coordinates.0 .1;
                    while counter_0 >= coordinates.1 .0 {
                        let list_point = heat_map.entry((counter_0, counter_1)).or_insert(0);
                        *list_point += 1;
                        counter_0 -= 1;
                        counter_1 -= 1;
                    }
                } else {
                    let mut counter_0 = coordinates.0 .0;
                    let mut counter_1 = coordinates.0 .1;
                    while counter_0 <= coordinates.1 .0 {
                        let list_point = heat_map.entry((counter_0, counter_1)).or_insert(0);
                        *list_point += 1;
                        counter_0 += 1;
                        counter_1 -= 1;
                    }
                }
            } else {
                if coordinates.0 .0 > coordinates.1 .0 {
                    let mut counter_0 = coordinates.0 .0;
                    let mut counter_1 = coordinates.0 .1;
                    while counter_0 >= coordinates.1 .0 {
                        let list_point = heat_map.entry((counter_0, counter_1)).or_insert(0);
                        *list_point += 1;
                        counter_0 -= 1;
                        counter_1 += 1;
                    }
                } else {
                    let mut counter_0 = coordinates.0 .0;
                    let mut counter_1 = coordinates.0 .1;
                    while counter_0 <= coordinates.1 .0 {
                        let list_point = heat_map.entry((counter_0, counter_1)).or_insert(0);
                        *list_point += 1;
                        counter_0 += 1;
                        counter_1 += 1;
                    }
                }
            }
        }
    }

    let mut result = 0;
    for (_index, value) in heat_map {
        if value > 1 {
            result += 1;
        }
    }

    result
}

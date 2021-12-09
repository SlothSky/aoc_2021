use ansi_term::Color::{Red, RGB};
use std::{collections::HashMap, f32::RADIX, fs};

pub fn day_09_main() {
    println!(
        "\n{}\n\t• 1 Fuel required by crabs: {}\n\t• n Fuel required by crabs: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 9:"),
        Red.paint(first_part_09().to_string()),
        Red.paint(second_part_09().to_string())
    )
}

fn first_part_09() -> u32 {
    let heightmap = fs::read_to_string("assets/09/heightmap.txt").unwrap();
    let mut heightmap_vec = Vec::new();

    for line in heightmap.lines() {
        let mut height_vec = Vec::new();
        for ch in line.chars() {
            height_vec.push(ch.to_digit(10).unwrap());
        }

        heightmap_vec.push(height_vec);
    }

    let mut results = Vec::new();
    for (line_index, line) in heightmap_vec.iter().enumerate() {
        if line_index == 0 {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                }
            }
        } else if line_index == heightmap_vec.len() - 1 {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                }
            }
        } else {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            results.push(ch);
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for r in results {
        result += r + 1
    }

    result
}

fn second_part_09() -> u32 {
    let heightmap = fs::read_to_string("assets/09/heightmap.txt").unwrap();
    let mut heightmap_vec = Vec::new();

    for line in heightmap.lines() {
        let mut height_vec = Vec::new();
        for ch in line.chars() {
            height_vec.push(ch.to_digit(10).unwrap());
        }

        heightmap_vec.push(height_vec);
    }

    let mut results = Vec::new();
    for (line_index, line) in heightmap_vec.iter().enumerate() {
        if line_index == 0 {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }
                            counter += 1;
                            results.push(counter);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }
                            counter += 1;
                            results.push(counter);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = ch_index;
                            while next != 9 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }
                            counter += 1;
                            results.push(counter);
                        }
                    }
                }
            }
        } else if line_index == heightmap_vec.len() - 1 {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }
                            counter += 1;
                            results.push(counter);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 && index_counter >= 1 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            counter += 1;
                            results.push(counter);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = ch_index;
                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 && index_counter >= 0 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            counter += 1;
                            results.push(counter);
                        }
                    }
                }
            }
        } else {
            for (ch_index, ch) in line.iter().enumerate() {
                match ch_index {
                    0 => {
                        if ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 && index_counter >= 1 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            counter += 1;
                            results.push(counter);
                        }
                    }
                    99 => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 && index_counter >= 1 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 && index_counter >= 1 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            counter += 1;
                            results.push(counter);
                        }
                    }
                    _ => {
                        if ch < line.get(ch_index - 1).unwrap()
                            && ch < line.get(ch_index + 1).unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                            && ch
                                < heightmap_vec
                                    .get(line_index + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap()
                        {
                            let mut counter = 0;
                            let mut next = 0;
                            let mut index_counter = ch_index;

                            while next != 9 && index_counter >= 1 {
                                next = *(line.get(index_counter - 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }

                            index_counter = ch_index;
                            while next != 9 {
                                next = *(line.get(index_counter + 1).unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter += 1;
                            }

                            index_counter = line_index;
                            while next != 9 {
                                next = *(heightmap_vec
                                    .get(index_counter + 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                            }

                            index_counter = line_index;
                            while next != 9 && index_counter >= 1 {
                                next = *(heightmap_vec
                                    .get(index_counter - 1)
                                    .unwrap()
                                    .get(ch_index)
                                    .unwrap());
                                if next != 9 {
                                    counter += 1;
                                }
                                index_counter -= 1;
                            }
                            counter += 1;
                            results.push(counter);
                        }
                    }
                }
            }
        }
    }

    results.sort_unstable();
    println!(" {:?}", results);
    results.get(results.len() - 1).unwrap()
        * results.get(results.len() - 2).unwrap()
        * results.get(results.len() - 3).unwrap()
}

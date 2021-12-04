use ansi_term::Color::{Red, RGB};
use std::fs;

#[derive(Debug)]
struct BingoField {
    first_line: [i32; 5],
    second_line: [i32; 5],
    third_line: [i32; 5],
    fourth_line: [i32; 5],
    fifth_line: [i32; 5],
}

impl BingoField {
    fn new() -> BingoField {
        BingoField {
            first_line: [-1, -1, -1, -1, -1],
            second_line: [-1, -1, -1, -1, -1],
            third_line: [-1, -1, -1, -1, -1],
            fourth_line: [-1, -1, -1, -1, -1],
            fifth_line: [-1, -1, -1, -1, -1],
        }
    }

    fn from(source: &BingoField) -> BingoField {
        let mut result = BingoField {
            first_line: [-1, -1, -1, -1, -1],
            second_line: [-1, -1, -1, -1, -1],
            third_line: [-1, -1, -1, -1, -1],
            fourth_line: [-1, -1, -1, -1, -1],
            fifth_line: [-1, -1, -1, -1, -1],
        };

        for (line_counter, line) in source.as_array().into_iter().enumerate() {
            for (index_counter, number) in line.into_iter().enumerate() {
                match line_counter {
                    0 => result.first_line[index_counter] = number,
                    1 => result.second_line[index_counter] = number,
                    2 => result.third_line[index_counter] = number,
                    3 => result.fourth_line[index_counter] = number,
                    4 => result.fifth_line[index_counter] = number,
                    _ => println!("Something went wrong, removing the number"),
                }
            }
        }

        result
    }

    pub fn as_array(&self) -> [[i32; 5]; 5] {
        [
            self.first_line,
            self.second_line,
            self.third_line,
            self.fourth_line,
            self.fifth_line,
        ]
    }

    fn check_for_bingo(&self) -> bool {
        for line in &self.as_array() {
            if line == &[-1; 5] {
                return true;
            };
        }

        for index in 0..5 {
            let mut vertical_check: [i32; 5] = [-1; 5];

            for (counter, line) in self.as_array().into_iter().enumerate() {
                vertical_check[counter] = line[index];
            }

            if vertical_check == [-1; 5] {
                return true;
            };
        }

        false
    }

    fn remove_number(&mut self, pick: i32) {
        for (line_counter, line) in self.as_array().into_iter().enumerate() {
            for (index_counter, number) in line.into_iter().enumerate() {
                if number == pick {
                    match line_counter {
                        0 => self.first_line[index_counter] = -1,
                        1 => self.second_line[index_counter] = -1,
                        2 => self.third_line[index_counter] = -1,
                        3 => self.fourth_line[index_counter] = -1,
                        4 => self.fifth_line[index_counter] = -1,
                        _ => println!("Something went wrong, removing the number"),
                    }
                }
            }
        }
    }
}

pub fn day_04_main() {
    let (first_win, cache) = first_part_04();

    println!(
        "\n{}\n\t• Bingo win: {}\n\t• Bingo loss: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 4:"),
        Red.paint(first_win.to_string()),
        Red.paint(cache.to_string())
    )
}

fn first_part_04() -> (i32, i32) {
    let random_numbers = fs::read_to_string("assets/04/random_numbers.txt").unwrap();
    let bingo_string = fs::read_to_string("assets/04/bingo_fields.txt").unwrap();
    let bingo_strings = bingo_string.replace("\r\n\r\n", "\n\n");
    let mut bingo_fields: Vec<(BingoField, bool)> = Vec::new();

    for string in bingo_strings.split_terminator("\n\n") {
        let mut field = BingoField::new();

        for (line_index, line) in string.lines().enumerate() {
            for (index, number) in line.split_whitespace().enumerate() {
                match line_index {
                    0 => field.first_line[index] = number.trim().parse::<i32>().unwrap(),
                    1 => field.second_line[index] = number.trim().parse::<i32>().unwrap(),
                    2 => field.third_line[index] = number.trim().parse::<i32>().unwrap(),
                    3 => field.fourth_line[index] = number.trim().parse::<i32>().unwrap(),
                    4 => field.fifth_line[index] = number.trim().parse::<i32>().unwrap(),
                    _ => println!("Unexpected line skipped"),
                }
            }
        }

        bingo_fields.push((field, false));
    }

    let mut cache = (BingoField::new(), -1);
    let mut first_win = (BingoField::new(), -1);

    let mut counter = 0;
    for number in random_numbers.split_terminator(',') {
        let number = number.trim().parse::<i32>().unwrap();

        for (field, bingoed) in &mut bingo_fields {
            field.remove_number(number);

            if field.check_for_bingo() && counter == 0 {
                counter += 1;
                first_win = (BingoField::from(field), number);
                *bingoed = true;
            } else if field.check_for_bingo() && !*bingoed {
                cache = (BingoField::from(field), number);
                *bingoed = true;
            }
        }
    }

    let first_win = calc_result(&first_win.0, first_win.1);
    let cache = calc_result(&cache.0, cache.1);

    (first_win, cache)
}

fn calc_result(field: &BingoField, number: i32) -> i32 {
    let mut result = 0;

    for line in field.as_array() {
        for number in line {
            if number != -1 {
                result += number;
            }
        }
    }

    number * result
}

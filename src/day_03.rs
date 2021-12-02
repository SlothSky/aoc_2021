use ansi_term::Color::{Red, RGB};
// use std::cmp;
// use std::fs::File;
// use std::io::Read;

pub fn day_03_main() {
    println!(
        "\n{}\n\t• {} increases\n\t• {} increases",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 3:"),
        Red.paint(first_part_03().to_string()),
        Red.paint(second_part_03().to_string())
    )
}

fn first_part_03() -> i32 {
    3
}

fn second_part_03() -> i32 {
    3
}

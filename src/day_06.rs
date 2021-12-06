use ansi_term::Color::{Red, RGB};
use plotters::{
    self,
    prelude::{
        BitMapBackend, ChartBuilder, DiscreteRanged, FontStyle, IntoDrawingArea, IntoLinspace,
        LineSeries,
    },
    style::{text_anchor::Pos, Color, FontDesc, ShapeStyle, TextStyle, BLACK, RED, WHITE},
};
use plotters_backend::BackendColor;
use std::fs;
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};

#[derive(Debug)]
struct Fish {
    timer: i32,
}

pub fn day_06_main() {
    /*
    println!(
        "\n{}\n\t• Amount of fish after 80 days: {}\n\t• Amount of fish after 256 days: {}",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 6:"),
        Red.paint(_first_part_06().to_string()),
        Red.paint(second_part_06().to_string())
    )*/
    let graph_string = fs::read_to_string("assets/06/init_graph.txt").unwrap();

    let mut stdout = stdout();
    println!("\n");
    for line in graph_string.lines() {
        stdout.queue(cursor::SavePosition);
        stdout.write(format!("{}\n", line).as_bytes());
        // stdout.queue(cursor::RestorePosition);
        stdout.flush();
        thread::sleep(time::Duration::from_millis(100));
    }
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

    for _day in 0..80 {
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
    let root_area =
        BitMapBackend::new("assets/06/plotters_fish.png", (1280, 720)).into_drawing_area();
    root_area
        .fill(&BLACK)
        .expect("Something went wrong with filling the root area");
    let root_area = root_area
        .titled("Fishes over time", TextStyle {
            color: BackendColor { alpha: 1.0, rgb: (206, 206, 206) },
            font: FontDesc::new(
                plotters_backend::FontFamily::Monospace, 
                30.0, 
                FontStyle::Normal,
            ),
            pos: Pos {
                h_pos: plotters_backend::text_anchor::HPos::Center,
                v_pos: plotters_backend::text_anchor::VPos::Center,
            }
        })
        .expect("Something went wrong with setting the root area's title.");
    let x_axis = (0f32..180f32).step(0.5);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .build_cartesian_2d(-2f32..182f32, -10000f32..2500000f32)
        .expect("Something went wrong while creating the ChartBuilder");

    cc.configure_mesh()
        .disable_mesh()
        .x_labels(20)
        .x_label_formatter(&|v| format!("{:}", v))
        .x_label_style(TextStyle {
            color: BackendColor {
                alpha: 1.0,
                rgb: (206, 206, 206),
            },
            font: FontDesc::new(
                plotters_backend::FontFamily::Monospace,
                12.0,
                FontStyle::Normal,
            ),
            pos: Pos {
                h_pos: plotters_backend::text_anchor::HPos::Center,
                v_pos: plotters_backend::text_anchor::VPos::Center,
            },
        })
        .y_labels(20)
        .y_label_formatter(&|v| format!("{:}", v))
        .y_label_style(TextStyle {
            color: BackendColor {
                alpha: 1.0,
                rgb: (206, 206, 206),
            },
            font: FontDesc::new(
                plotters_backend::FontFamily::Monospace,
                12.0,
                FontStyle::Normal,
            ),
            pos: Pos {
                h_pos: plotters_backend::text_anchor::HPos::Center,
                v_pos: plotters_backend::text_anchor::VPos::Center,
            },
        })
        .axis_style(ShapeStyle {
            color: WHITE.to_rgba(),
            filled: true,
            stroke_width: 1,
        })
        .draw()
        .expect("Something went wrong while drawing the diagram");

    let init_fish_string = fs::read_to_string("assets/06/init_fish_list.txt").unwrap();
    let mut fish_list = [0; 9];

    for fish_string in init_fish_string.split_terminator(',') {
        fish_list[fish_string.parse::<i64>().unwrap() as usize] += 1;
    }

    for day in 0..256 {
        if day < 180 {
            cc.draw_series(LineSeries::new(
                x_axis
                    .values()
                    .map(|_x| (day as f32, ((count_fishes(fish_list) / 1000) as f32))),
                RED,
            ))
            .expect("Something went wrong while drawing the data.");
        }

        fish_list.rotate_left(1);
        fish_list[6] += fish_list[8];
    }

    let fish_counter = count_fishes(fish_list);

    fish_counter
}

fn count_fishes(fish_list: [i64; 9]) -> i64 {
    let mut fish_counter = 0;
    for fishes in fish_list {
        fish_counter += fishes;
    }

    fish_counter
}

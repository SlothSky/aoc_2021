/* 
    • ATTENTION: DO NOT LOOK DIRECTLY INTO THIS CODE NOR RUN IT

    • FOLLOWING CODE MIGHT SEVERLY DAMAGE YOUR EYES!!!

    • THIS CODE IS DEFINETLY AGAINST ANY CONVENTION (INCLUDING GENEVA CONVENTIONS)

    • IT IS DANGEROUS TO GO BELOW THIS LINE, TAKE THIS:

           *      
          ///     
          ///     
          ///     
          ***     
       // ### //   
          ###
          ###             

*/

/*

    !!! YOU HAVE BEEN WARNED !!! 

*/



use ansi_term::Color::{Red, RGB};
use std::cmp::Ordering;
// use std::cmp;
use std::fs;
use std::collections::HashMap;
// use std::io::Read;

pub fn day_03_main() {
    println!(
        "\n{}\n\t• {} increases\n\t• {} increases",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 3:"),
        Red.paint("First rule about day 3: You don't talk about day 3."),
        Red.paint("Second rule about day 3: You don't talk about day 3.")
    )
}

fn first_part_03() -> i32 {
    let diag_data = fs::read_to_string("assets/03/diagnostic_data.txt").unwrap();
    let mut counter_map = HashMap::new();

    for line in diag_data.lines() {
       for (bit_pos , value )in line.trim().chars().enumerate() {
           if value == '0' {
            let count = counter_map.entry(bit_pos).or_insert(0);
            *count += 1;
           }
       }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..counter_map.len() {
        if counter_map.get(&i).unwrap().cmp(&500) == Ordering::Greater {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    println!("{}, {}", gamma_rate, epsilon_rate);
    3
}

fn second_part_03_ox() -> i32 {
    let diag_data = fs::read_to_string("assets/03/diagnostic_data.txt").unwrap();
    let mut diag_data_vec = Vec::new();
    let mut counter_map = HashMap::new();

    for line in diag_data.lines() {
        diag_data_vec.push(line.trim());
    }

    for line in diag_data.lines() {
       for (bit_pos , value )in line.trim().chars().enumerate() {
           if value == '0' {
            let count = counter_map.entry(bit_pos).or_insert(0);
            *count += 1;
           }
       }
    }

    
    while diag_data_vec.len() > 1 {
        for index in 0..counter_map.len() {
            
            let mut counter_map = HashMap::new();
            for line in &mut diag_data_vec {
                for (bit_pos , value )in line.trim().chars().enumerate() {
                    if value == '0' {
                     let count = counter_map.entry(bit_pos).or_insert(0);
                     *count += 1;
                    }
                }
            }

            println!("BIT INDEX: {}", index);
            println!("{:?}", diag_data_vec);
            let mut most = '0';
            if &(diag_data_vec.len() / 2) < counter_map.get(&index).unwrap() {
                most = '0';
            } else {
                most = '1';
            }
            println!("MODST: {}", counter_map.get(&index).unwrap());
            let mut deletion_list = Vec::new();
            let mut counter = 0;

            for row in &diag_data_vec {
            for (bit, value) in row.chars().enumerate() {
                    if bit == index && value == most {
                        break;
                    } else if bit == index && value != most {
                        deletion_list.push(counter);
                    } else {
                        continue;
                    }
                }
                counter += 1;
            }
            deletion_list.sort_by(|a, b| b.cmp(a));
            for del in deletion_list {
                diag_data_vec.remove(del);
            }
        }
    }

    3
}


fn second_part_03_co2() -> i32 {
    let diag_data = fs::read_to_string("assets/03/diagnostic_data.txt").unwrap();
    let mut diag_data_vec = Vec::new();
    let mut counter_map = HashMap::new();

    for line in diag_data.lines() {
        diag_data_vec.push(line.trim());
    }

    for line in diag_data.lines() {
       for (bit_pos , value )in line.trim().chars().enumerate() {
           if value == '0' {
            let count = counter_map.entry(bit_pos).or_insert(0);
            *count += 1;
           }
       }
    }

    while diag_data_vec.len() > 1 {
        for index in 0..counter_map.len() {
            
            let mut counter_map = HashMap::new();
            for line in &mut diag_data_vec {
                for (bit_pos , value )in line.trim().chars().enumerate() {
                    if value == '0' {
                     let count = counter_map.entry(bit_pos).or_insert(0);
                     *count += 1;
                    }
                }
            }

            println!("BIT INDEX: {}", index);
            println!("{:?}", diag_data_vec);
            let mut most = '0';
            if counter_map.get(&index).unwrap() >  &(diag_data_vec.len() / 2) {
                most = '1';
            } else {
                most = '0';
            }
            println!("MODST: {}", counter_map.get(&index).unwrap());
            let mut deletion_list = Vec::new();
            let mut counter = 0;

            for row in &diag_data_vec {
            for (bit, value) in row.chars().enumerate() {
                    if bit == index && value == most {
                        break;
                    } else if bit == index && value != most {
                        deletion_list.push(counter);
                    } else {
                        continue;
                    }
                }
                counter += 1;
            }

            deletion_list.sort_by(|a, b| b.cmp(a));
            for del in deletion_list {
                diag_data_vec.remove(del);
            }

        }
    }

    3
}
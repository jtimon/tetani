//! The ui module contains convience for user interface.

use std::io;

/// Asks the user for a u32 between input_min and input_max and returns it.
pub fn input_u32(input_min: u32, input_max: u32) -> u32 {

    let mut input_value;
    loop {
        input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("get_input: Failed to read line");

        let input_value: u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input has to be u32");
                continue;
            },
        };
        if input_value < input_min {
            println!("Min input {}", input_min);
            continue;
        }
        if input_value > input_max {
            println!("Max input {}", input_max);
            continue;
        }
        return input_value;
    }
}

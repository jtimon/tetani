use std::io;

use tetani::digital::{
    BinaryTask,
    BinOp,
    ProgrammableLogicArray,
};

use tetani::genetic::{
    Individual,
    Population,
    Task,
};

fn main() {
    println!("Let's operate with 2 binary vectors, how many bits?");
    let vector_size : usize = get_input(1, 8) as usize;
    println!("What binary operation? 0: AND, 1: OR, 2: XOR 3: NOR");
    let operation_type : BinOp = input_2binop(get_input(0, 3));

    let bin_task = BinaryTask::new(vector_size, operation_type);

    println!("----------------------------------------------------------");
    println!("Fenotypes:");
    println!("MAX  fitness: {}", vector_size);
    let population_size = 4;
    let input_len = bin_task.input.len();
    let mut pop : Population<ProgrammableLogicArray, BinaryTask> = Population::new(bin_task, population_size);

    // pop.add_and_rate_individual(ProgrammableLogicArray::new_null(input_len, vector_size));
    pop.add_and_rate_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));
    pop.add_and_rate_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));
    pop.add_and_rate_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 4));
    pop.add_and_rate_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 8));
    // pop.add_and_rate_individual(ProgrammableLogicArray::new_rand(input_len, vector_size));

    println!("----------------------------------------------------------");
    println!("Genotypes:");
    pop.print();
}

pub fn input_2binop(input : u32) -> BinOp {
    match input {
        0 => BinOp::AND,
        1 => BinOp::OR,
        2 => BinOp::XOR,
        3 => BinOp::NOR,
        _ => panic!("crash and burn"),
    }
}

fn get_input(input_min: u32, input_max: u32) -> u32 {

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

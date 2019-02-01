use std::io;

// TODO Do better
use tetani::*;

fn main() {
    println!("Let's operate with 2 binary vectors, how many bits?");
    let vector_size : usize = get_input(1, 8) as usize;
    println!("What binary operation? 0: AND, 1: OR, 2: XOR 3: NOR");
    let operation_type : BinOp = input_2binop(get_input(0, 3));

    let v_a = get_null_bitvector(vector_size);
    let v_b = get_rand_bitvector(vector_size);
    let v_result = calculate_result(&operation_type, &v_a, &v_b);
    assert_eq!(vector_size as i32, calculate_fitness_result(&v_result, &v_result));

    let mut input = v_a.clone();
    input.extend(&v_b);
    let input_len = input.len();
    assert_eq!(input_len, vector_size * 2);

    let population_size = 4;
    let mut pla: Vec<ProgrammableLogicArray> = Vec::with_capacity(population_size);

    pla.push(ProgrammableLogicArray::new_null(input_len, vector_size));
    pla.push(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 10));
    pla.push(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32));
    pla.push(ProgrammableLogicArray::new_rand(input_len, vector_size));

    let mut pla_results: Vec<Vec<bool>> = Vec::with_capacity(population_size);
    for i in 0..population_size {
        println!("----------------------------------------------------------");
        println!("Individual {}:", i);
        pla_results.push(pla[i].calculate_output(&input));
    }

    println!("----------------------------------------------------------");
    println!("Genotypes:");
    for individual in pla {
        individual.print();
    }
    println!("----------------------------------------------------------");
    println!("Fenotypes:");
    println!("MAX  fitness: {}", vector_size);
    for i in 0..population_size {
        println!("PLA{} fitness: {}", i, calculate_fitness_result(&v_result, &pla_results[i]));
    }

    println!("----------------------------------------------------------");
    println!("Your choice was {} bits and operation {}", vector_size, binop_2str(operation_type));
    println!("----------------------------------------------------------");
    print!("input:  "); print_bitvector(&input);
    print!("A:      "); print_bitvector(&v_a);
    print!("B:      "); print_bitvector(&v_b);
    print!("RESULT: "); print_bitvector(&v_result);
    for i in 0..population_size {
        print!("PLA{}:   ", i); print_bitvector(&pla_results[i]);
    }
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

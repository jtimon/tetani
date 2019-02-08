use std::io;

use tetani::digital::{
    BinOp,
    BinaryTask,
    ProgrammableLogicArray,
    TruthTable,
    u32_2binop,
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
    let operation_type : BinOp = u32_2binop(get_input(0, 3));

    let bin_task = BinaryTask::new(vector_size, operation_type);

    println!("----------------------------------------------------------");
    println!("MAX  fitness: {}", bin_task.get_max_fitness());
    let population_size = 4;
    let input_len = vector_size * 2;

    let mut tt_pop : Population<TruthTable, BinaryTask> = Population::new(bin_task.clone(), population_size);
    tt_pop.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 2));
    tt_pop.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 4));
    tt_pop.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 8));
    tt_pop.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 16));

    let mut pop : Population<ProgrammableLogicArray, BinaryTask> = Population::new(bin_task, population_size);
    pop.add_unrated_individual(ProgrammableLogicArray::new_null(input_len, vector_size));
    pop.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));
    pop.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 4));
    pop.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 8));
    // pop.add_unrated_individual(ProgrammableLogicArray::new_rand(input_len, vector_size));

    let mut individual = TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 2);
    let max_tries = 4;
    for _ in 0..max_tries {
        let mut mutant = individual.clone();
        mutant.mutate();
        tt_pop.add_and_rate_individual(mutant);
    }
    tt_pop.add_and_rate_individual(individual);

    tt_pop.rate_unrated_individuals();
    pop.rate_unrated_individuals();

    pop.add_and_rate_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));

    println!("----------------------------------------------------------");
    tt_pop.print();
    pop.print();
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

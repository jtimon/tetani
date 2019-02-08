
use tetani::digital::{
    BinOp,
    BinaryTask,
    ProgrammableLogicArray,
    TruthTable,
    binop_2str,
    u32_2binop,
};
use tetani::genetic::{
    Individual,
    Population,
    Task,
};
use tetani::ui;

fn main() {
    println!("Let's operate with 2 binary vectors, how many bits?");
    let vector_size : usize = ui::input_u32(1, 8) as usize;
    println!("What binary operation? 0: AND, 1: OR, 2: XOR 3: NOR");
    let operation_type : BinOp = u32_2binop(ui::input_u32(0, 3));
    // println!("How many maximum individuals generated per population?");
    // let max_generation : usize = ui::input_u32(1, 10000) as usize;
    let max_generation : usize = 5000;
    let population_size = 3000;

    println!("----------------------------------------------------------");
    let bin_task = BinaryTask::new(vector_size, operation_type.clone());
    println!("Your choice was {} bits and operation {}. Max generation: {}", vector_size, binop_2str(&operation_type), max_generation);
    println!("MAX  fitness: {}", bin_task.get_max_fitness());
    let input_len = vector_size * 2;

    println!("----------------------------------------------------------");
    let mut pop_tt : Population<TruthTable, BinaryTask> = Population::new(bin_task.clone(), population_size);
    pop_tt.add_unrated_individual(TruthTable::new_null(input_len, vector_size));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 2));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 4));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 8));
    pop_tt.rate_unrated_individuals();

    println!("Initial Truth Table population:");
    pop_tt.print();
    println!("----------------------------------------------------------");
    println!("Truth Table population:");

    while pop_tt.task.get_max_fitness() > pop_tt.best_fitness() && pop_tt.len() < max_generation {
        let mut mutant = pop_tt.best().clone();
        mutant.mutate();
        pop_tt.add_and_rate_individual(mutant);
    }
    // pop_tt.print();
    println!("Truth Table total generations: {}", pop_tt.len());
    println!("Best Truth Table:");
    pop_tt.best().print();

    println!("----------------------------------------------------------");
    let mut pop_pla : Population<ProgrammableLogicArray, BinaryTask> = Population::new(bin_task, population_size);
    pop_pla.add_unrated_individual(ProgrammableLogicArray::new_null(input_len, vector_size));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 4));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 8));
    pop_pla.rate_unrated_individuals();

    println!("Initial Programable Logic Array population:");
    pop_pla.print();
    println!("----------------------------------------------------------");
    println!("Programable Logic Array population:");

    while pop_pla.task.get_max_fitness() > pop_pla.best_fitness() && pop_pla.len() < max_generation {
        let mut mutant = pop_pla.best().clone();
        mutant.mutate();
        pop_pla.add_and_rate_individual(mutant);
    }
    // pop_pla.print();
    println!("Programable Logic Array total generations: {}", pop_pla.len());
    println!("Best Programable Logic Array:");
    pop_pla.best().print();
}

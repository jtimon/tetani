
use tetani::digital::{
    BinOp,
    BinaryIndividual,
    ProgrammableLogicArray,
    TruthTable,
};
use tetani::genetic::{
    ImitationTask,
    Individual,
    Population,
    Task,
};
use tetani::ui;

fn main() {
    println!("Let's operate with 2 binary vectors, how many bits?");
    let vector_size : usize = ui::input_u32(1, 8) as usize;
    println!("What binary operation (logic gate)? 0: AND, 1: OR, 2: XOR, 3: NAND, 4: NOR, 5: NXOR");
    let operation_type : BinOp = BinOp::from_u32(ui::input_u32(0, 5));
    // println!("How many maximum individuals generated per population?");
    // let max_generation : usize = ui::input_u32(1, 10000) as usize;
    let max_generation : usize = 5000;
    let population_size = 3000;

    println!("----------------------------------------------------------");
    let bi = BinaryIndividual::new(operation_type.clone(), vector_size * 2);
    let bin_task = ImitationTask::new(bi);
    println!("Your choice was {} bits and operation {}. Max generation: {}", vector_size, operation_type.to_str(), max_generation);
    println!("MAX  fitness: {}", bin_task.max_fitness());
    let input_len = vector_size * 2;

    println!("----------------------------------------------------------");
    let mut pop_tt : Population<TruthTable, ImitationTask<BinaryIndividual>> = Population::new(bin_task.clone(), population_size);

    pop_tt.add_unrated_individual(TruthTable::new_null(input_len, vector_size));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 2));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 4));
    // pop_tt.add_unrated_individual(TruthTable::new_muta(input_len, vector_size, vector_size as u32 * 8));
    pop_tt.rate_unrated_individuals();

    println!("Initial Truth Table population:");
    pop_tt.print();
    println!("----------------------------------------------------------");
    println!("Truth Table population:");

    pop_tt.learn_task(max_generation);
    // pop_tt.print();
    println!("Truth Table total generations: {}", pop_tt.len());
    println!("Best Truth Table:");
    pop_tt.best().print();

    println!("----------------------------------------------------------");
    let mut pop_pla : Population<ProgrammableLogicArray, ImitationTask<BinaryIndividual>> = Population::new(bin_task, population_size);

    pop_pla.add_unrated_individual(ProgrammableLogicArray::new_null(input_len, vector_size));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 2));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 4));
    // pop_pla.add_unrated_individual(ProgrammableLogicArray::new_mutated(input_len, vector_size, vector_size as u32 * 8));
    pop_pla.rate_unrated_individuals();

    println!("Initial Programable Logic Array population:");
    pop_pla.print();
    println!("----------------------------------------------------------");
    println!("Programable Logic Array population:");

    pop_pla.learn_task(max_generation);
    // pop_pla.print();
    println!("Programable Logic Array total generations: {}", pop_pla.len());
    println!("Best Programable Logic Array:");
    pop_pla.best().print();
}

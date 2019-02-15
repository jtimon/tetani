
/// Reverse engineer a truth tables using genetic algorithms and an emulation of a Programmable Logic Array
/// Individual ProgrammableLogicArray can learn to perfectly imitate a TruthTable

use tetani::digital::{
    ProgrammableLogicArray,
    TruthTable,
};
use tetani::genetic::{
    ImitationTask,
    Individual,
    Population,
    Task,
};

fn pla_can_learn_tt(in_size: usize, out_size: usize, max_generation: usize) {

    let tt = TruthTable::new_rand(in_size, out_size);

    println!("----------------------------------------------------------");
    println!("Target Truth Table:");
    tt.print();
    let tt_imitation_task = ImitationTask::new(tt);
    let max_fitness = tt_imitation_task.max_fitness();
    println!("Max fitness: {}, Max generation: {}", max_fitness, max_generation);

    println!("----------------------------------------------------------");
    let indi = ProgrammableLogicArray::new_null(in_size, out_size);
    let mut pop_pla : Population<ProgrammableLogicArray, ImitationTask<TruthTable>> = Population::new(tt_imitation_task, max_generation);

    pop_pla.add_unrated_individual(indi);
    pop_pla.rate_unrated_individuals();

    println!("Initial Truth Table population:");
    pop_pla.print();
    println!("----------------------------------------------------------");
    println!("Truth Table population:");

    pop_pla.learn_task(max_generation);
    // pop_pla.print();
    println!("Truth Table total generations: {}", pop_pla.len());
    println!("Best Truth Table:");
    pop_pla.best().print();
    assert_eq!(max_fitness, pop_pla.best_fitness());
}

#[test]
fn pla_can_learn_tt_in1_out1() {
    for _ in 0..10000 {
        pla_can_learn_tt(1, 1, 30);
    }
}

#[test]
fn pla_can_learn_tt_in1_out2() {
    for _ in 0..10000 {
        pla_can_learn_tt(1, 2, 60);
    }
}

#[test]
fn pla_can_learn_tt_in2_out2() {
    for _ in 0..10000 {
        pla_can_learn_tt(2, 2, 200);
    }
}

#[test]
fn pla_can_learn_tt_in2_out1() {
    for _ in 0..10000 {
        pla_can_learn_tt(2, 1, 100);
    }
}

// #[test]
// fn pla_can_learn_tt_in3_out1() {
//     for _ in 0..1 {
//         pla_can_learn_tt(3, 1, 900000);
//     }
// }

// #[test]
// fn pla_can_learn_tt_in4_out1() {
//     for _ in 0..1 {
//         pla_can_learn_tt(4, 1, 1000000);
//     }
// }

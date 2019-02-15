
/// Individual ProgrammableLogicArray can learn to perfectly imitate BinaryIndividual

use tetani::digital::{
    BinOp,
    BinaryIndividual,
    ProgrammableLogicArray,
};
use tetani::genetic::{
    ImitationTask,
    Individual,
    Population,
    Task,
};

fn pla_can_learn_bi(operation_type: BinOp, vector_size: usize, max_generation: usize) {

    let max_fitness = vector_size as i32 * 2i32.pow(vector_size as u32 * 2);

    let bi = BinaryIndividual::new(operation_type, vector_size * 2);
    bi.print();
    let bin_task = ImitationTask::new(bi);
    println!("Max fitness: {}, Max generation: {}", bin_task.max_fitness(), max_generation);

    println!("----------------------------------------------------------");
    let indi = ProgrammableLogicArray::new_null(vector_size * 2, vector_size);
    let mut pop_tt : Population<ProgrammableLogicArray, ImitationTask<BinaryIndividual>> = Population::new(bin_task, max_generation);

    pop_tt.add_unrated_individual(indi);
    pop_tt.rate_unrated_individuals();

    println!("Initial Programmable Logic Array population:");
    pop_tt.print();
    println!("----------------------------------------------------------");
    println!("Programmable Logic Array population:");

    pop_tt.learn_task(max_generation);
    // pop_tt.print();
    println!("Programmable Logic Array total generations: {}", pop_tt.len());
    println!("Best Programmable Logic Array:");
    pop_tt.best().print();
    assert_eq!(pop_tt.best_fitness(), max_fitness);
}

#[test]
fn pla_can_learn_bi_and1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::AND, 1, 120);
    }
}

#[test]
fn pla_can_learn_bi_or1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::OR, 1, 60);
    }
}

#[test]
fn pla_can_learn_bi_xor1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::XOR, 1, 120);
    }
}

#[test]
fn pla_can_learn_bi_nand1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::NAND, 1, 70);
    }
}

#[test]
fn pla_can_learn_bi_nor1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::NOR, 1, 130);
    }
}

#[test]
fn pla_can_learn_bi_xnor1() {
    for _ in 0..10 {
        pla_can_learn_bi(BinOp::XNOR, 1, 130);
    }
}


// #[test]
// fn pla_can_learn_bi_or2() {
//     for _ in 0..100 {
//         pla_can_learn_bi(BinOp::OR, 2, 100000);
//     }
// }

// #[test]
// fn pla_can_learn_bi_nand2() {
//     for _ in 0..100 {
//         pla_can_learn_bi(BinOp::NAND, 2, 100000);
//     }
// }

#[test]
fn pla_can_learn_bi_and2() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::AND, 2, 500);
    }
}

#[test]
fn pla_can_learn_bi_xor2() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::XOR, 2, 500);
    }
}

#[test]
fn pla_can_learn_bi_nor2() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::NOR, 2, 500);
    }
}

#[test]
fn pla_can_learn_bi_xnor2() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::XNOR, 2, 500);
    }
}

// #[test]
// fn pla_can_learn_bi_or3() {
//     for _ in 0..100 {
//         pla_can_learn_bi(BinOp::OR, 3, 2000);
//     }
// }

// #[test]
// fn pla_can_learn_bi_nand3() {
//     for _ in 0..100 {
//         pla_can_learn_bi(BinOp::NAND, 3, 1500);
//     }
// }

#[test]
fn pla_can_learn_bi_and3() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::AND, 3, 1500);
    }
}

#[test]
fn pla_can_learn_bi_xor3() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::XOR, 3, 1500);
    }
}

#[test]
fn pla_can_learn_bi_nor3() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::NOR, 3, 2000);
    }
}

#[test]
fn pla_can_learn_bi_xnor3() {
    for _ in 0..1 {
        pla_can_learn_bi(BinOp::XNOR, 3, 1500);
    }
}


/// Individual TruthTable can learn to perfectly imitate BinaryIndividual

use tetani::digital::{
    BinOp,
    BinaryIndividual,
    TruthTable,
};
use tetani::genetic::{
    ImitationTask,
    Individual,
    Population,
    Task,
};

fn tt_can_learn_bi(operation_type: BinOp, vector_size: usize, max_generation: usize) {

    let max_fitness = vector_size as i32 * 2i32.pow(vector_size as u32 * 2);

    let bi = BinaryIndividual::new(operation_type, vector_size * 2);
    bi.print();
    let bin_task = ImitationTask::new(bi);
    println!("Max fitness: {}, Max generation: {}", bin_task.max_fitness(), max_generation);

    println!("----------------------------------------------------------");
    let indi = TruthTable::new_null(vector_size * 2, vector_size);
    let mut pop_tt : Population<TruthTable, ImitationTask<BinaryIndividual>> = Population::new(bin_task, max_generation);

    pop_tt.add_unrated_individual(indi);
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
    assert_eq!(pop_tt.best_fitness(), max_fitness);
}

#[test]
fn tt_can_learn_bi_and1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::AND, 1, 60);
    }
}

#[test]
fn tt_can_learn_bi_or1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::OR, 1, 70);
    }
}

#[test]
fn tt_can_learn_bi_xor1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::XOR, 1, 70);
    }
}

#[test]
fn tt_can_learn_bi_nand1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::NAND, 1, 70);
    }
}

#[test]
fn tt_can_learn_bi_nor1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::NOR, 1, 60);
    }
}

#[test]
fn tt_can_learn_bi_xnor1() {
    for _ in 0..100000 {
        tt_can_learn_bi(BinOp::XNOR, 1, 70);
    }
}

#[test]
fn tt_can_learn_bi_and2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::AND, 2, 400);
    }
}

#[test]
fn tt_can_learn_bi_or2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::OR, 2, 500);
    }
}

#[test]
fn tt_can_learn_bi_xor2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::XOR, 2, 500);
    }
}

#[test]
fn tt_can_learn_bi_nand2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::NAND, 2, 400);
    }
}

#[test]
fn tt_can_learn_bi_nor2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::NOR, 2, 400);
    }
}

#[test]
fn tt_can_learn_bi_xnor2() {
    for _ in 0..100 {
        tt_can_learn_bi(BinOp::XNOR, 2, 400);
    }
}


#[test]
fn tt_can_learn_bi_and3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::AND, 3, 2000);
    }
}

#[test]
fn tt_can_learn_bi_or3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::OR, 3, 2500);
    }
}

#[test]
fn tt_can_learn_bi_xor3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::XOR, 3, 2500);
    }
}

#[test]
fn tt_can_learn_bi_nand3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::NAND, 3, 2500);
    }
}

#[test]
fn tt_can_learn_bi_nor3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::NOR, 3, 2000);
    }
}

#[test]
fn tt_can_learn_bi_xnor3() {
    for _ in 0..2 {
        tt_can_learn_bi(BinOp::XNOR, 3, 2500);
    }
}


/// Individual NeuralNetwork can learn to perfectly imitate BinaryIndividual

use tetani::digital::{
    BinOp,
    BinaryIndividual,
};
use tetani::genetic::{
    ImitationTask,
    Individual,
    Population,
    Task,
};
use tetani::neural::{
    NeuralNetwork,
};

fn nn_can_learn_bi(operation_type: BinOp, net_depth: usize, vector_size: usize, max_generation: usize) {

    let max_fitness = vector_size as i32 * 2i32.pow(vector_size as u32 * 2);

    let bi = BinaryIndividual::new(operation_type, vector_size * 2);
    bi.print();
    let bin_task = ImitationTask::new(bi);
    println!("Max fitness: {}, Max generation: {}", bin_task.max_fitness(), max_generation);

    println!("----------------------------------------------------------");
    let indi = NeuralNetwork::new_null(vector_size * 2, vector_size, net_depth);
    let mut pop_nn : Population<NeuralNetwork, ImitationTask<BinaryIndividual>> = Population::new(bin_task, max_generation);

    pop_nn.add_unrated_individual(indi);
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, vector_size as u32));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, 3));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, 3));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, 3));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, 3));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, vector_size as u32 * 2));
    pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, vector_size as u32 * 4));
    // pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, vector_size as u32 * 8));
    // pop_nn.add_unrated_individual(NeuralNetwork::new_muta(vector_size * 2, vector_size, net_depth, vector_size as u32 * 16));
    pop_nn.rate_unrated_individuals();

    pop_nn.set_num_selection_truncation(4);
    println!("Initial Neural Network population:");
    // pop_nn.print();
    println!("----------------------------------------------------------");
    println!("Neural Network population:");

    pop_nn.learn_task(max_generation);
    // pop_nn.print();
    println!("Neural Network total generations: {}", pop_nn.len());
    println!("Best Neural Network:");
    pop_nn.best().print();
    assert_eq!(pop_nn.best_fitness(), max_fitness);
}

#[test]
fn nn_can_learn_bi_or1() {
    for _ in 0..2 {
        nn_can_learn_bi(BinOp::OR, 1, 1, 100000);
    }
}

// #[test]
// fn nn_can_learn_bi_and1() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::AND, 1, 1, 100000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xor1() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::XOR, 2, 1, 100000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nand1() {
//     for _ in 0..10 {
//         nn_can_learn_bi(BinOp::NAND, 1, 1, 100000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nor1() {
//     for _ in 0..10 {
//         nn_can_learn_bi(BinOp::NOR, 1, 1, 100000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xnor1() {
//     for _ in 0..10 {
//         nn_can_learn_bi(BinOp::XNOR, 1, 1, 100000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_and2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::AND, 1, 2, 400);
//     }
// }

// #[test]
// fn nn_can_learn_bi_or2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::OR, 1, 2, 500);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xor2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::XOR, 2, 2, 500);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nand2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::NAND, 1, 2, 400);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nor2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::NOR, 1, 2, 400);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xnor2() {
//     for _ in 0..100 {
//         nn_can_learn_bi(BinOp::XNOR, 1, 2, 400);
//     }
// }


// #[test]
// fn nn_can_learn_bi_and3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::AND, 1, 3, 2000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_or3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::OR, 1, 3, 2500);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xor3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::XOR, 2, 3, 2500);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nand3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::NAND, 1, 3, 2500);
//     }
// }

// #[test]
// fn nn_can_learn_bi_nor3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::NOR, 1, 3, 2000);
//     }
// }

// #[test]
// fn nn_can_learn_bi_xnor3() {
//     for _ in 0..2 {
//         nn_can_learn_bi(BinOp::XNOR, 1, 3, 2500);
//     }
// }

//! The genetic module is mostly to praise Darwin and Wallace. It implements Genetic Algorithms.

use crate::digital::{
    get_null_bitvector,
    increment_bitvector,
};

/// Individuals compete for fitness within a Population
pub trait Individual {
    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool>;
    fn mutate(&mut self);
    fn print(&self);
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}

/// Individuals are rated fitness for a given task
pub trait Task {
    fn calculate_fitness(&self, individual: &Individual) -> i32;
    fn max_fitness(&self) -> i32;
}

struct RatedIndividual<I: Individual> {
    indi: I,
    fitness: i32,
}

impl<I> RatedIndividual<I>
    where I: Individual {
    pub fn print(&self) {
        println!("Fitness {}, individual:", self.fitness);
        // self.indi.print();
    }
}

/// Some data struct ordered by fitness (repeated values are allowed) should replace Vec
pub struct Population<I: Individual, T: Task> {
    pub task: T,
    /// TODO This list should be ordered by fitness
    pop: Vec< RatedIndividual<I> >,
    /// Same initial capacity as pop for now
    unrated_pop: Vec<I>,
    best_index: usize,
    best_fitness: i32,
}

impl<I, T> Population<I, T>
    where I: Individual + 'static + Clone, T: Task {

    pub fn new(task: T, capacity: usize) -> Population<I, T> {
        let pop : Vec< RatedIndividual<I> > = Vec::with_capacity(capacity);
        let unrated_pop : Vec<I> = Vec::with_capacity(capacity);
        Population{
            task,
            pop,
            unrated_pop,
            best_fitness: -1000,
            best_index: 0,
        }
    }

    fn update_best(&mut self, new_fitness: i32) {
        if new_fitness > self.best_fitness {
            self.best_fitness = new_fitness;
            self.best_index = self.pop.len();
        }
    }

    pub fn len(&self) -> usize {
        self.pop.len()
    }

    pub fn best_fitness(&self) -> i32 {
        self.best_fitness
    }

    pub fn best(&self) -> &I {
        &self.pop[self.best_index].indi
    }

    fn add_rated_individual(&mut self, indi: RatedIndividual<I>) {
        self.update_best(indi.fitness);
        self.pop.push(indi);
    }

    pub fn add_unrated_individual(&mut self, indi: I) {
        self.unrated_pop.push(indi);
    }

    pub fn add_and_rate_individual(&mut self, indi: I) {
        let fitness = self.task.calculate_fitness(&indi);
        self.add_rated_individual(RatedIndividual{indi, fitness});
    }

    pub fn next_generation(&mut self) {
        let mut mutant = self.best().clone();
        mutant.mutate();
        self.add_and_rate_individual(mutant);
    }

    pub fn learn_task(&mut self, max_generation: usize) {
        while self.best_fitness() < self.task.max_fitness() && self.pop.len() < max_generation {
            self.next_generation();
        }
    }

    pub fn rate_unrated_individuals(&mut self) {
        let mut new_best = self.best_fitness();
        let mut new_best_index = self.best_index;
        for indi in self.unrated_pop.iter() {
            let fitness = self.task.calculate_fitness(indi);
            if fitness > new_best {
                new_best = fitness;
                new_best_index = self.pop.len();
            }
            self.pop.push(RatedIndividual{
                indi: indi.clone(),
                fitness,
            });
        }
        self.best_fitness = new_best;
        self.best_index = new_best_index;
        self.unrated_pop.clear();
    }

    pub fn print(&self) {
        for indi in self.pop.iter() {
            indi.print();
        }
    }
}

fn calculate_fitness_result(result: &Vec<bool>, v_tested: &Vec<bool>) -> i32 {
    assert_eq!(result.len(), v_tested.len());
    let mut fitness = 0;
    for i in 0..result.len() {
        if result[i] == v_tested[i] {
            fitness += 1;
        }
    }
    fitness
}

/// Task to imitate another individual, even if it's a different species/type than the population that evolves to imitate it
/// The inidividual must be stateless, that is, not having an internal state that can affect calculate_output.
/// Note that in neural networks having recursion implies having an internal state.
///
/// # Examples
///
/// ```
/// use tetani::digital::BinOp;
/// use tetani::digital::BinaryIndividual;
/// use tetani::genetic::ImitationTask;
/// let bi_and_2 = BinaryIndividual::new(BinOp::AND, 2);
/// let bi_xor_2 = BinaryIndividual::new(BinOp::XOR, 2);
/// let imitate_bi_and_2 = ImitationTask::new(bi_and_2);
/// let imitate_bi_xor_2 = ImitationTask::new(bi_xor_2);
/// ```
#[derive(Debug)]
pub struct ImitationTask<I: Individual> {
    indi: I,
}

impl<I> ImitationTask<I>
    where I: Individual + 'static + Clone {

    pub fn new(indi: I) -> ImitationTask<I> {
        ImitationTask {
            indi,
        }
    }
}

impl<I> Clone for ImitationTask<I>
    where I: Individual + 'static + Clone {

    fn clone(&self) -> ImitationTask<I> {
        ImitationTask {
            indi: self.indi.clone(),
        }
    }
}

impl<I> Task for ImitationTask<I>
    where I: Individual + 'static + Clone {

    fn calculate_fitness(&self, other: &Individual) -> i32 {
        assert_eq!(self.indi.output_size(), other.output_size());
        let in_size = self.indi.input_size();
        assert_eq!(in_size, other.input_size());

        let mut fitness = 0;
        let mut input = get_null_bitvector(in_size);
        let input_space_cardinality = 2usize.pow(in_size as u32);

        for j in 0..input_space_cardinality {
            let output_self = self.indi.calculate_output(&input);
            let output_other = other.calculate_output(&input);
            fitness += calculate_fitness_result(&output_self, &output_other);
            // println!("----------------------------------------------------------");
            // print!("input:  "); print_bitvector(&input);
            // print!("A:      "); print_bitvector(&input[0..self.indi.input_size() / 2]);
            // print!("B:      "); print_bitvector(&input[self.indi.input_size() / 2..self.indi.input_size()]);
            // print!("INDI:   "); print_bitvector(&output_self);
            // print!("OTHER:   "); print_bitvector(&output_other);
            // print!("FITNESS: {}", fitness);

            if j < input_space_cardinality - 1 {
                increment_bitvector(&mut input);
            }
        }

        fitness
    }

    fn max_fitness(&self) -> i32 {
        self.indi.output_size() as i32 * 2i32.pow(self.indi.input_size() as u32)
    }
}

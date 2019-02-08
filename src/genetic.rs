//! The genetic module is mostly to praise Darwin and Wallace.

/// Individuals compete for fitness within a Population
pub trait Individual {
    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool>;
    fn random_mutation(&mut self);
    fn print(&self);
}

/// Individuals are rated fitness for a given task
pub trait Task {
    fn calculate_fitness(&self, individual: &Individual) -> i32;
}

pub struct RatedIndividual<I: Individual> {
    indi: I,
    fitness: i32,
}

impl<I> RatedIndividual<I>
    where I: Individual {
    pub fn print(&self) {
        println!("Fitness {}, individual:", self.fitness);
        self.indi.print();
    }
}

/// Some data struct ordered by fitness (repeated values are allowed) should replace Vec
pub struct Population<I: Individual, T: Task> {
    task: T,
    /// TODO This list should be ordered by fitness
    pop: Vec< RatedIndividual<I> >,
    /// Same initial capacity as pop for now
    unrated_pop: Vec<I>,
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
        }
    }

    pub fn add_rated_individual(&mut self, indi: RatedIndividual<I>) {
        self.pop.push(indi);
    }

    pub fn add_unrated_individual(&mut self, indi: I) {
        self.unrated_pop.push(indi);
    }

    pub fn add_and_rate_individual(&mut self, indi: I) {
        let fitness = self.task.calculate_fitness(&indi);
        self.add_rated_individual(RatedIndividual{indi, fitness});
    }

    pub fn rate_unrated_individuals(&mut self) {
        for indi in self.unrated_pop.iter() {
            let fitness = self.task.calculate_fitness(indi);
            self.pop.push(RatedIndividual{
                indi: indi.clone(),
                fitness,
            });
        }
        self.unrated_pop.clear();
    }

    pub fn print(&self) {
        for indi in self.pop.iter() {
            indi.print();
        }
    }
}

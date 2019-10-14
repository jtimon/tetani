//! The universe module is a generic interface to simulate models where time only goes forwards.

/// A Universe can calculate repeatedly its next state from its own state in the current step (aka turn).
/// A turn or step can be defined as the minimal amount of time measurable in the model implementing the Universe trait.
pub trait Universe {
    fn next_turn(&mut self);

    fn run_for(&mut self, turns: usize) {
        for _ in 0..turns {
            self.next_turn();
        }
    }
}

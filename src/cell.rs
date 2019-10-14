//! The cell module is about cellular automata.

use crate::universe::{
    Universe,
};

/// Cellular automaton repeateadly calculating their boolean output from its boolean neightbours' previous output
/// neightbours: 3 (including itself, left, and right)
/// https://en.wikipedia.org/wiki/Elementary_cellular_automaton
///
/// # Examples
///
/// ```
/// use tetani::cell::ElementaryCellularAutomata;
/// let wolfram0 = ElementaryCellularAutomata::new_by_rule(0);
/// let wolfram255 = ElementaryCellularAutomata::new_by_rule(255);
/// ```

// fn rule_neightbours2result(rule: u8, neightbours: Vec<bool>) -> bool {
//     assert!(neightbours.len() == 3);
//     false
// }

#[derive(Debug)]
pub struct ElementaryCellularAutomata {
    rule: u8,
    cells: Vec<bool>,
}

impl ElementaryCellularAutomata {

    pub fn new_by_rule(rule: u8) -> ElementaryCellularAutomata {
        let eca = ElementaryCellularAutomata {
            rule: rule,
            cells: vec![false],
        };

        eca
    }

    fn print_cells(&self) {
        print!("-------");
        for c in self.cells.iter() {
            if *c {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!("-------");
    }

    fn get_value_in_new_pos_system(&self, new_pos: usize) -> bool {
        let current_cells_len = self.cells.len();
        if new_pos <= 0 || new_pos >= current_cells_len {
            return false;
        }

        self.cells[new_pos - 1]
    }

    fn transition_from_neightbours(&self, new_pos: usize) -> bool {
        let mut neightbours = Vec::with_capacity(3);
        for pos in new_pos-1..new_pos+1 {
            neightbours.push(self.get_value_in_new_pos_system(pos));
        }

        false
        // rule_neightbours2result(self.rule, neightbours)
    }

    fn transition(&mut self) {
        let current_cells_len = self.cells.len();
        let next_cells_len = current_cells_len + 2;
        let mut new_cells = Vec::with_capacity(next_cells_len);
        for new_pos in 0..next_cells_len {
            new_cells.push(self.transition_from_neightbours(new_pos));
        }
        self.cells = new_cells;
    }
}

impl Universe for ElementaryCellularAutomata {

    fn next_turn(&mut self) {
        self.print_cells();
        self.transition();
    }
}

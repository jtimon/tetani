
/// Test wolfram's 1d and binary cellular automata anonically defined from 0 to 255
/// https://en.wikipedia.org/wiki/Elementary_cellular_automaton
/// https://en.wikipedia.org/wiki/Wolfram_code

use tetani::cell::{
    ElementaryCellularAutomata,
};
use tetani::universe::{
    Universe,
};

fn create_by_rule_and_run_for(rule: u8, turns: usize) {
    let mut eca = ElementaryCellularAutomata::new_by_rule(rule);
    eca.run_for(turns);
}

#[test]
fn cell_rule_from_0_to_255_for_300_turns() {
    for rule in 0..255 {
        create_by_rule_and_run_for(rule, 300);
    }
}

#[test]
fn cell_rule_from_0_to_255_for_1000_turns() {
    for rule in 0..255 {
        create_by_rule_and_run_for(rule, 300);
    }
}

#[test]
fn cell_255_for_300() {
   create_by_rule_and_run_for(255, 300);
}

#[test]
fn cell_28_for_300() {
    create_by_rule_and_run_for(28, 300);
}

#[test]
fn cell_50_for_300() {
    create_by_rule_and_run_for(50, 300);
}

#[test]
fn cell_54_for_300() {
    create_by_rule_and_run_for(54, 300);
}

#[test]
fn cell_60_for_300() {
    create_by_rule_and_run_for(60, 300);
}

#[test]
fn cell_90_for_300() {
    create_by_rule_and_run_for(90, 300);
}

#[test]
fn cell_94_for_300() {
    create_by_rule_and_run_for(94, 300);
}

#[test]
fn cell_102_for_300() {
    create_by_rule_and_run_for(102, 300);
}

#[test]
fn cell_110_for_300() {
    create_by_rule_and_run_for(110, 300);
}

#[test]
fn cell_150_for_300() {
    create_by_rule_and_run_for(150, 300);
}

#[test]
fn cell_158_for_300() {
    create_by_rule_and_run_for(158, 300);
}

#[test]
fn cell_188_for_300() {
    create_by_rule_and_run_for(188, 300);
}

#[test]
fn cell_190_for_300() {
    create_by_rule_and_run_for(190, 300);
}

#[test]
fn cell_220_for_300() {
    create_by_rule_and_run_for(220, 300);
}

#[test]
fn cell_222_for_300() {
    create_by_rule_and_run_for(222, 300);
}

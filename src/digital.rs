//! The digital module is about boolean functions and digital systems.

use rand::Rng;

pub use crate::genetic::{
    Individual,
    Task,
};

/// Basic logic gates. Operations that can be done between two bits.
#[derive(Clone, PartialEq, Debug)]
pub enum BinOp {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}

impl BinOp {
    pub fn from_u32(input : u32) -> BinOp {
        match input {
            0 => BinOp::AND,
            1 => BinOp::OR,
            2 => BinOp::XOR,
            3 => BinOp::NAND,
            4 => BinOp::NOR,
            5 => BinOp::XNOR,
            _ => unreachable!(),
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            BinOp::AND => "AND",
            BinOp::OR => "OR",
            BinOp::XOR => "XOR",
            BinOp::NAND => "NAND",
            BinOp::NOR => "NOR",
            BinOp::XNOR => "XNOR",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum AndPseudoMatrixValue {
    NEITHER,
    REQUIRED,
    COMPLEMENT,
}

impl AndPseudoMatrixValue {
    fn new_null(vector_size: usize) -> Vec<AndPseudoMatrixValue> {
        let mut v: Vec<AndPseudoMatrixValue> = Vec::with_capacity(vector_size);
        for _i in 0..vector_size {
            v.push(AndPseudoMatrixValue::NEITHER);
        }
        v
    }
}

fn increment_and_row(and_row: &mut Vec<AndPseudoMatrixValue>) {
    for i in 0..and_row.len() {
        if and_row[i] == AndPseudoMatrixValue::NEITHER {
            and_row[i] = AndPseudoMatrixValue::REQUIRED;
            // if we set a NEITHER to REQUIRED, we set all the previous COMPLEMENT to NEITHER
            for j in 0..i {
                and_row[j] = AndPseudoMatrixValue::NEITHER;
            }
            return;
        } else if and_row[i] == AndPseudoMatrixValue::REQUIRED {
            and_row[i] = AndPseudoMatrixValue::COMPLEMENT;
            // if we set a REQUIRED to COMPLEMENT, we set all the previous COMPLEMENT to NEITHER
            for j in 0..i {
                and_row[j] = AndPseudoMatrixValue::NEITHER;
            }
            return;
        }
    }
    panic!("AndPseudoMatrixValue vector overflow");
}

fn and_func_to_str(and_row: &Vec<AndPseudoMatrixValue>) -> String {
    let mut to_return = String::new();

    for i in 0..and_row.len() {
        if and_row[i] == AndPseudoMatrixValue::REQUIRED {
            to_return.push_str(&format!("a{}", i));
        } else if and_row[i] == AndPseudoMatrixValue::COMPLEMENT {
            to_return.push_str(&format!("a{}'", i));
        }
    }

    to_return
}

fn bitvec2minterm_str(and_row: &Vec<bool>) -> String {
    let mut to_return = String::new();

    for i in 0..and_row.len() {
        if and_row[i] == true {
            to_return.push_str(&format!("a{}", i));
        } else {
            to_return.push_str(&format!("a{}'", i));
        }
    }

    to_return
}

fn bitvec2maxterm(and_row: &Vec<bool>) -> String {
    let mut to_return = String::new();
    to_return.push_str("(");
    let mut first_added = true;

    for i in 0..and_row.len() {
        if first_added {
            first_added = false;
        } else {
            to_return.push_str(" + ");
        }
        // The opposite of minterms by convention
        if and_row[i] == false {
            to_return.push_str(&format!("a{}", i));
        } else {
            to_return.push_str(&format!("a{}'", i));
        }
    }
    to_return.push_str(")");

    to_return
}

// returns true if all input requirements described in the and row are satisfied, false otherwise
fn compare_and_row(and_row: &Vec<AndPseudoMatrixValue>, input: &Vec<bool>) -> bool {
    assert_eq!(and_row.len(), input.len());
    let mut to_return = false;
    for i in 0..and_row.len() {
        if and_row[i] != AndPseudoMatrixValue::NEITHER {
            // if the whole row is neither we still have to return false
            to_return = true;
        }
        if (and_row[i] == AndPseudoMatrixValue::REQUIRED && !input[i]) ||
            (and_row[i] == AndPseudoMatrixValue::COMPLEMENT && input[i]) {
                return false
        }
    }
    assert!(to_return); // this should never be false since we're skiping the null and row (all NEITHER)
    to_return
}

/// Programmable Logic Array:
///
/// https://en.wikipedia.org/wiki/Programmable_logic_array
///
/// Instead of having an and matrix of 2^(in_size * 2) x in_size, we'll have one with 3^in_size x in_size.
/// Because any and row in which the bits for a given input and its complement can be ignored.
/// we can also -1 because we can ignore the null vector.
///
/// In fact, there's no need to store it anywhere, it's just a matrix with all possible combinations of inputs,
/// which can be cheaply reproduced programatically by simply iterating.
///
/// TODO: use the minimal representation, this one still has redundancies.
/// For example: out0 = (a0) + (a0 + a1), it's equivalent to just out0 = (a0).
/// This results in redudant different genotypes that result in equivalent fenotypes.
#[derive(Debug)]
pub struct ProgrammableLogicArray {
    /// No need to store the input, pass it as a reference to calculate_output.
    /// The input is expected to be Vec<bool> with the same size as stored or it will panic.
    in_size: usize,
    /// or_matrix: [[bool; 3^in_size-1]; out_size],
    /// imaginary_and_matrix: [[bool; in_size]; 3^in_size-1],
    or_matrix: Vec< Vec<bool> >,
}

impl Clone for ProgrammableLogicArray {
    fn clone(&self) -> ProgrammableLogicArray {
        ProgrammableLogicArray {
            or_matrix: self.or_matrix.clone(),
            in_size: self.in_size,
        }
    }
}

impl ProgrammableLogicArray {

    // aka calculate_and_row_size, since they are the same
    fn calculate_or_column_size(in_size: usize) -> usize {
        3usize.pow(in_size as u32) - 1
    }

    pub fn new_null(in_size: usize, out_size: usize) -> ProgrammableLogicArray {
        let mut pla = ProgrammableLogicArray {
            in_size: in_size,
            or_matrix: Vec::with_capacity(out_size),
        };
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(in_size);
        for _i in 0..out_size {
            pla.or_matrix.push(get_null_bitvector(or_column_size));
        }
        pla
    }

    pub fn new_rand(in_size: usize, out_size: usize) -> ProgrammableLogicArray {
        let mut pla = ProgrammableLogicArray {
            in_size: in_size,
            or_matrix: Vec::with_capacity(out_size),
        };
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(in_size);
        for _i in 0..out_size {
            pla.or_matrix.push(get_rand_bitvector(or_column_size));
        }
        pla
    }

    pub fn new_mutated(in_size: usize, out_size: usize, num_mutations: u32) -> ProgrammableLogicArray {
        let mut pla = ProgrammableLogicArray {
            in_size: in_size,
            or_matrix: Vec::with_capacity(out_size),
        };
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(in_size);
        for _i in 0..out_size {
            pla.or_matrix.push(get_null_bitvector(or_column_size));
        }
        for _i in 0..num_mutations {
            pla.mutate();
        }
        pla
    }

    pub fn print2(&self) {
        let max_bitvector_print = 32;
        for i in 0..self.or_matrix.len() {
            print!("or matrix column {}:", i);
            print_limited_bitvector(&self.or_matrix[i], max_bitvector_print);
        }
    }

}

impl Individual for ProgrammableLogicArray {
    fn mutate(&mut self) {
        let chosen_bit = rand::thread_rng().gen_range(0, self.or_matrix.len());
        let chosen_output = rand::thread_rng().gen_range(0, self.or_matrix[0].len());
        self.or_matrix[chosen_bit][chosen_output] = !self.or_matrix[chosen_bit][chosen_output];
    }

    fn print(&self) {
        let out_size = self.or_matrix.len();
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(self.in_size);
        let mut and_matrix_row = AndPseudoMatrixValue::new_null(self.in_size);
        increment_and_row(&mut and_matrix_row); // skip null vector (all NEITHER)

        let mut output_functions : Vec<String> = Vec::with_capacity(out_size);
        let mut first_added : Vec<bool> = Vec::with_capacity(out_size);
        for _i in 0..out_size {
            output_functions.push(String::new());
            first_added.push(true);
        }

        for j in 0..or_column_size {
            let and_function = and_func_to_str(&and_matrix_row);
            for i in 0..out_size {
                if self.or_matrix[i][j] {
                    if first_added[i] {
                        first_added[i] = false;
                    } else {
                        output_functions[i].push_str(" + ");
                    }
                    output_functions[i].push_str(&format!("{}", and_function));
                }
            }

            if j < or_column_size - 1 {
                increment_and_row(&mut and_matrix_row);
            }
        }

        for i in 0..out_size {
            println!("out{} = {}", i, output_functions[i]);
        }
    }

    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool> {
        assert_eq!(self.in_size, input.len());
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(self.in_size);

        let mut and_matrix_row = AndPseudoMatrixValue::new_null(self.in_size);
        increment_and_row(&mut and_matrix_row); // skip null vector (all NEITHER)

        let out_size = self.or_matrix.len();
        let mut output : Vec<bool> = Vec::with_capacity(out_size);
        for _i in 0..out_size {
            output.push(false);
        }

        for j in 0..or_column_size {

            for i in 0..out_size {
                if !output[i] && self.or_matrix[i][j] && compare_and_row(&and_matrix_row, &input) {
                    output[i] = true;
                }
            }

            if j < or_column_size - 1 {
                increment_and_row(&mut and_matrix_row);
            }
        }

        output
    }

    fn output_size(&self) -> usize {
        self.or_matrix.len()
    }

    fn input_size(&self) -> usize {
        self.in_size
    }
}

// returns true if all active bits in bitvector are also active in other or false otherwise
fn compare_and(bitvector: &Vec<bool>, other: &Vec<bool>) -> bool {
    assert_eq!(bitvector.len(), other.len());
    let mut to_return = false;
    for i in 0..bitvector.len() {
        if bitvector[i] {
            to_return = true;
            if !other[i] {
                return false
            }
        }
    }
    to_return
}

fn eq_bitvector(bitvector: &Vec<bool>, other: &Vec<bool>) -> bool {
    assert_eq!(bitvector.len(), other.len());
    for i in 0..bitvector.len() {
        if bitvector[i] != other[i] {
            return false
        }
    }
    true
}

pub fn calculate_result(operation_type : &BinOp, input: &Vec<bool>) -> Vec<bool> {
    assert_eq!(input.len() % 2, 0);
    let half_size = input.len() / 2;
    let mut result: Vec<bool> = Vec::with_capacity(half_size);
    for i in 0..half_size {
        result.push(
            match operation_type {
                BinOp::AND =>    input[i] && input[i + half_size],
                BinOp::OR  =>    input[i] || input[i + half_size],
                BinOp::XOR =>    input[i] != input[i + half_size],
                BinOp::NOR  => !(input[i] || input[i + half_size]),
                BinOp::NAND => !(input[i] && input[i + half_size]),
                BinOp::XNOR =>   input[i] == input[i + half_size],
            }
        );
    }
    result
}

pub fn increment_bitvector(bitvector: &mut Vec<bool>) {
    for i in 0..bitvector.len() {
        if !bitvector[i] {
            bitvector[i] = true;
            for j in 0..i {
                bitvector[j] = false;
            }
            return;
        }
    }
    panic!("bitvector overflow");
}

pub fn get_null_bitvector(vector_size: usize) -> Vec<bool> {
    let mut v: Vec<bool> = Vec::with_capacity(vector_size);
    for _i in 0..vector_size {
        v.push(false);
    }
    v
}

pub fn get_rand_bitvector(vector_size: usize) -> Vec<bool> {
    let mut v: Vec<bool> = Vec::with_capacity(vector_size);
    for _i in 0..vector_size {
        v.push(rand::thread_rng().gen_range(0, 2) > 0);
    }
    v
}

pub fn print_bitvector(v: &[bool]) {
    for &i in v {
        let bit_str : &str = if i {
            "1"
        } else {
            "0"
        };
        print!("{} ", bit_str);
    }
    println!("");
}

fn print_limited_bitvector(v: &[bool], max: usize) {
    for i in 0..v.len() {
        print!("{} ", if v[i] {
            "1"
        } else {
            "0"
        });
        if i > max {
            print!("...({} bits)", v.len());
            break;
        }
    }
    println!("");
}

/// An individual implementing a single binary operation between two bitvectors of equal len.
/// Thus its output must be half its input.
///
/// # Examples
///
/// ```
/// use tetani::digital::BinOp;
/// use tetani::digital::BinaryIndividual;
/// let bi_and = BinaryIndividual::new(BinOp::AND, 2);
/// let bi_or = BinaryIndividual::new(BinOp::OR, 2);
/// let bi_xor = BinaryIndividual::new(BinOp::XOR, 2);
/// ```
#[derive(Debug)]
pub struct BinaryIndividual {
    operation_type : BinOp,
    out_size : usize,
    in_cardinality : usize,
}

impl BinaryIndividual {
    pub fn new(operation_type : BinOp, in_size : usize) -> BinaryIndividual {
        assert_eq!(0, in_size % 2);
        BinaryIndividual {
            operation_type,
            out_size : in_size / 2,
            in_cardinality : 2usize.pow(in_size as u32),
        }
    }

    /// input cardinality
    fn in_cardinality(&self) -> usize {
        self.in_cardinality
    }
}

impl Clone for BinaryIndividual {
    fn clone(&self) -> BinaryIndividual {
        BinaryIndividual {
            operation_type : self.operation_type.clone(),
            out_size : self.out_size,
            in_cardinality : self.in_cardinality,
        }
    }
}

impl Individual for BinaryIndividual {

    /// unimplemented! This individual is not supposed to evolve, but only to be imitated others using ImitationTask.
    fn mutate(&mut self) {
        unimplemented!();
    }

    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool> {
        assert_eq!(input.len(), self.input_size());
        calculate_result(&self.operation_type, &input)
    }

    fn print(&self) {
        println!("BinaryIndividual: operation_type: {}, in_size: {}, out_size: {}, in_cardinality: {}",
                 self.operation_type.to_str(),
                 self.input_size(),
                 self.output_size(),
                 self.in_cardinality(),
        );
    }

    fn output_size(&self) -> usize {
        self.out_size
    }

    /// input size is twice as much as the output
    fn input_size(&self) -> usize {
        self.out_size * 2
    }
}

/// https://en.wikipedia.org/wiki/Truth_table
/// https://en.wikipedia.org/wiki/Canonical_normal_form
///
/// # Examples
///
/// ```
/// use tetani::digital::TruthTable;
/// let tt_empt = TruthTable::new_empt(1);
/// let tt_null = TruthTable::new_null(1, 1);
/// let tt_rand = TruthTable::new_rand(1, 1);
/// let tt_muta = TruthTable::new_muta(1, 1, 1);
/// ```
#[derive(Debug)]
pub struct TruthTable {
    /// The inputs part of the table is reproduced programatically when needed by simply iterating the input bitvector.
    /// outputs: [[bool; 2^in_size]; out_size]
    outputs: Vec< Vec<bool> >,
    /// We know in_size is the root square of any of the bitvetors inside outputs,
    /// but it is redundantly stored here for convinience.
    in_size: usize,
}

impl Clone for TruthTable {
    fn clone(&self) -> TruthTable {
        TruthTable {
            outputs: self.outputs.clone(),
            in_size: self.in_size,
        }
    }
}

impl TruthTable {

    pub fn new_empt(in_size: usize) -> TruthTable {
        TruthTable {
            in_size: in_size,
            outputs: vec![],
        }
    }

    pub fn new_null(in_size: usize, out_size: usize) -> TruthTable {
        let mut tt = TruthTable {
            in_size: in_size,
            outputs: Vec::with_capacity(out_size),
        };
        let column_size = tt.get_input_space_cardinality();
        for _i in 0..out_size {
            tt.outputs.push(get_null_bitvector(column_size));
        }
        tt
    }

    pub fn new_rand(in_size: usize, out_size: usize) -> TruthTable {
        let mut tt = TruthTable {
            in_size: in_size,
            outputs: Vec::with_capacity(out_size),
        };
        let column_size = tt.get_input_space_cardinality();
        for _i in 0..out_size {
            tt.outputs.push(get_rand_bitvector(column_size));
        }
        tt
    }

    pub fn new_muta(in_size: usize, out_size: usize, num_mutations: u32) -> TruthTable {
        let mut tt = TruthTable {
            in_size: in_size,
            outputs: Vec::with_capacity(out_size),
        };
        let column_size = tt.get_input_space_cardinality();
        for _i in 0..out_size {
            tt.outputs.push(get_null_bitvector(column_size));
        }
        for _i in 0..num_mutations {
            tt.mutate();
        }
        tt
    }

    fn get_input_space_cardinality(&self) -> usize {
        2usize.pow(self.in_size as u32)
    }
}

impl Individual for TruthTable {
    fn mutate(&mut self) {
        let chosen_output = rand::thread_rng().gen_range(0, self.outputs.len());
        let chosen_input_combination = rand::thread_rng().gen_range(0, self.outputs[0].len());
        self.outputs[chosen_output][chosen_input_combination] = !self.outputs[chosen_output][chosen_input_combination];
    }

    fn print(&self) {
        let out_size = self.outputs.len();
        let column_size = self.get_input_space_cardinality();
        let mut minterm = get_null_bitvector(self.in_size);

        let mut output_functions : Vec<String> = Vec::with_capacity(out_size);
        let mut first_added : Vec<bool> = Vec::with_capacity(out_size);
        for _i in 0..out_size {
            output_functions.push(String::new());
            first_added.push(true);
        }

        for j in 0..column_size {
            let minterm_str = bitvec2minterm_str(&minterm);
            for i in 0..out_size {
                if self.outputs[i][j] {
                    if first_added[i] {
                        first_added[i] = false;
                    } else {
                        output_functions[i].push_str(" + ");
                    }
                    output_functions[i].push_str(&format!("{}", minterm_str));
                }
            }

            if j < column_size - 1 {
                increment_bitvector(&mut minterm);
            }
        }

        for i in 0..out_size {
            println!("out{} = {}", i, output_functions[i]);
        }
    }

    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool> {
        assert_eq!(self.in_size, input.len());
        let out_size = self.outputs.len();
        let mut output : Vec<bool> = Vec::with_capacity(out_size);
        for _i in 0..out_size {
            output.push(false);
        }

        let column_size = self.get_input_space_cardinality();
        let mut minterm = get_null_bitvector(self.in_size);
        for j in 0..column_size {

            for i in 0..out_size {
                if !output[i] && self.outputs[i][j] && eq_bitvector(&minterm, &input) {
                    output[i] = true;
                }
            }

            if j < column_size - 1 {
                increment_bitvector(&mut minterm);
            }
        }

        output
    }

    fn output_size(&self) -> usize {
        self.outputs.len()
    }

    fn input_size(&self) -> usize {
        self.in_size
    }
}

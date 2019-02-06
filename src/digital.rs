use rand::Rng;

pub use crate::genetic::Individual;

pub enum BinOp {
    AND,
    OR,
    XOR,
    NOR,
}

pub fn binop_2str<'a>(input : BinOp) -> &'a str {
    match input {
        BinOp::AND => "AND",
        BinOp::OR => "OR",
        BinOp::XOR => "XOR",
        BinOp::NOR => "NOR",
    }
}

#[derive(PartialEq, Debug)]
enum AndPseudoMatrixValue {
    NEITHER,
    REQUIRED,
    COMPLEMENT,
}

fn get_null_and_row(vector_size: usize) -> Vec<AndPseudoMatrixValue> {
    let mut v: Vec<AndPseudoMatrixValue> = Vec::with_capacity(vector_size);
    for _i in 0..vector_size {
        v.push(AndPseudoMatrixValue::NEITHER);
    }
    v
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
    let mut first_added = true;

    for i in 0..and_row.len() {
        if and_row[i] != AndPseudoMatrixValue::NEITHER {
            if first_added {
                first_added = false;
            } else {
                to_return.push_str(" ⋅ ");
            }
        }
        if and_row[i] == AndPseudoMatrixValue::REQUIRED {
            to_return.push_str(&format!("a{}", i));
        } else if and_row[i] == AndPseudoMatrixValue::COMPLEMENT {
            to_return.push_str(&format!("¬a{}", i));
        }
    }

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
pub struct ProgrammableLogicArray {
    /// No need to store the input, pass it as a reference to calculate_output.
    /// The input is expected to be Vec<bool> with the same size as stored or it will panic.
    in_size: usize,
    /// or_matrix: [[bool; 3^in_size-1]; out_size],
    /// imaginary_and_matrix: [[bool; in_size]; 3^in_size-1],
    or_matrix: Vec< Vec<bool> >,
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
            pla.random_mutation();
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
    fn random_mutation(&mut self) {
        let chosen_bit = rand::thread_rng().gen_range(0, self.or_matrix.len());
        let chosen_output = rand::thread_rng().gen_range(0, self.or_matrix[0].len());
        self.or_matrix[chosen_bit][chosen_output] = !self.or_matrix[chosen_bit][chosen_output];
    }

    fn print(&self) {
        let out_size = self.or_matrix.len();
        let or_column_size = ProgrammableLogicArray::calculate_or_column_size(self.in_size);
        let mut and_matrix_row = get_null_and_row(self.in_size);
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
                    output_functions[i].push_str(&format!("({})", and_function));
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

        let mut and_matrix_row = get_null_and_row(self.in_size);
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

pub fn calculate_result(operation_type : &BinOp, v_a: &Vec<bool>, v_b: &Vec<bool>) -> Vec<bool> {
    assert_eq!(v_a.len(), v_b.len());
    let mut v_result: Vec<bool> = Vec::new();
    for i in 0..v_a.len() {
        v_result.push(
            match operation_type {
                BinOp::AND => v_a[i] && v_b[i],
                BinOp::OR => v_a[i] || v_b[i],
                BinOp::XOR => v_a[i] != v_b[i],
                BinOp::NOR => v_a[i] == v_b[i],
            }
        );
    }
    v_result
}

pub fn calculate_fitness_result(v_result: &Vec<bool>, v_tested: &Vec<bool>) -> i32 {
    assert_eq!(v_result.len(), v_tested.len());
    let mut fitness = 0;
    for i in 0..v_result.len() {
        if v_result[i] == v_tested[i] {
            fitness += 1;
        }
    }
    fitness
}

fn increment_bitvector(bitvector: &mut Vec<bool>) {
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
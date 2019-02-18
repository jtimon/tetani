//! The neural module implements Evolutionary Artificial Neural Networks (EANNs).

use rand::Rng;

pub use crate::genetic::Individual;

#[derive(Debug)]
struct Layer {
    pub in_size: usize,
    weights: Vec<Vec<i8>>,
    thresholds: Vec<i8>,
}

impl Layer {

    pub fn new(in_size: usize, out_size: usize) -> Layer {
        let mut lay = Layer {
            in_size,
            weights: Vec::with_capacity(out_size),
            thresholds: Vec::with_capacity(out_size),
        };

        for i in 0..out_size {
            lay.thresholds.push(0);
            lay.weights.push(Vec::with_capacity(in_size));
            for _ in 0..in_size {
                lay.weights[i].push(0);
            }
        }

        lay
    }

    fn mutate(&mut self) {
        let chosen_output = rand::thread_rng().gen_range(0, self.output_size());
        let chosen_input = rand::thread_rng().gen_range(0, self.in_size + 1);
        let max_mutation = 4;
        let mut mutation = rand::thread_rng().gen_range(1, max_mutation + 1);
        // Half the times, make the connection weaker rather than stronger
        if rand::thread_rng().gen_range(0, 2) > 0 {
            mutation = -mutation;
        }
        // Also mutate threshols some times
        if chosen_input == self.in_size {
            self.thresholds[chosen_output] += mutation;
        } else {
            self.weights[chosen_output][chosen_input] += mutation;
        }
    }

    fn output_size(&self) -> usize {
        self.thresholds.len()
    }

    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool> {
        assert_eq!(self.in_size, input.len());
        let mut output : Vec<bool> = Vec::with_capacity(self.output_size());

        for i in 0..self.output_size() {
            let mut result : i32 = 0;
            for j in 0..self.in_size {
                if input[j] {
                    result += self.weights[i][j] as i32;
                }
            }
            result -= self.thresholds[i] as i32;
            output.push(result > 0);
        }

        output
    }
}

impl Clone for Layer {
    fn clone(&self) -> Layer {
        Layer {
            in_size: self.in_size,
            weights: self.weights.clone(),
            thresholds: self.thresholds.clone(),
        }
    }
}

/// Artificial Neural Network
#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {

    pub fn new_null(in_size: usize, out_size: usize, net_depth: usize) -> NeuralNetwork {
        let mut nn = NeuralNetwork {
            layers: vec![],
        };

        for _ in 0..net_depth - 1 {
            nn.layers.push(Layer::new(in_size, in_size));
        }
        nn.layers.push(Layer::new(in_size, out_size));

        nn
    }

    pub fn new_muta(in_size: usize, out_size: usize, net_depth: usize, num_mutations: u32) -> NeuralNetwork {
        let mut nn = NeuralNetwork {
            layers: vec![],
        };

        for _ in 0..net_depth - 1 {
            nn.layers.push(Layer::new(in_size, in_size));
        }
        nn.layers.push(Layer::new(in_size, out_size));

        for _ in 0..num_mutations {
            nn.mutate();
        }

        nn
    }
}

impl Clone for NeuralNetwork {
    fn clone(&self) -> NeuralNetwork {
        NeuralNetwork {
            layers: self.layers.clone(),
        }
    }
}

impl Individual for NeuralNetwork {

    fn print(&self) {
        println!("NeuralNetwork (net_depth {})", self.layers.len());
    }

    fn mutate(&mut self) {
        let chosen_layer = rand::thread_rng().gen_range(0, self.layers.len());
        self.layers[chosen_layer].mutate();
    }

    fn calculate_output(&self, input: &Vec<bool>) -> Vec<bool> {
        let num_layers = self.layers.len();
        assert!(num_layers > 0);
        assert_eq!(self.layers[0].in_size, input.len());
        let mut outputs : Vec<Vec<bool>> = Vec::with_capacity(num_layers);

        outputs.push(self.layers[0].calculate_output(&input));
        for i in 0..num_layers - 1 {
            assert_eq!(self.layers[i+1].in_size, outputs[i].len());
            outputs.push(self.layers[i+1].calculate_output(&outputs[i]));
        }

        outputs[num_layers - 1].clone()
    }

    fn input_size(&self) -> usize {
        assert!(self.layers.len() > 0);
        self.layers[0].in_size
    }

    fn output_size(&self) -> usize {
        assert!(self.layers.len() > 0);
        self.layers[self.layers.len() - 1].thresholds.len()
    }
}

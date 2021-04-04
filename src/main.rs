#[macro_use]
extern crate timeit;

mod neuron;
mod synapse;

use neuron::Neuron;
use neuron::functions::NeuronType;
use rayon::prelude::*;

fn main() {
    let sec = timeit_loops!(1000000, {
        (0..16384).into_par_iter().for_each(|_| {
            let neuron_type: NeuronType = rand::random();
            let neuron: Neuron = Neuron { function: neuron_type, synapses: Vec::new(), resistance: 0.0 };
        });
    });

    println!("Took {:?} seconds to create 1,000,000 neurons with 16,384 synapses each", sec * 1000000.0)
}
extern crate utils;

use std::cmp::max;
use utils::math::sigmoid;
use super::neuron::Neuron;

pub struct Synapse {
    input: Neuron,
    output: Neuron,
    weight: f64,
    learning_rate: f64,
    activations: u8,
    activation_steps: u8,
    maximum: f64,
    minimum: f64,
}

impl Synapse {

    pub fn execute(&mut self, value: f64) {
        if value > self.weight {
            self.output.execute(value, self.weight);
            self.adjust_weight(value);
            self.increase_activations();
        } else {
            self.decrease_activations();
        }
    }

    fn adjust_weight(&mut self, value: f64) {
        self.weight += value * self.get_learning_rate()
    }

    fn increase_activations(&mut self) {
        self.activations += 1;
    }
    
    fn decrease_activations(&mut self) {
        self.activation_steps += 1;
        
        if self.activation_steps == 10 {
            self.activation_steps = 0;
            self.activations -= 1;
            self.activations = max(0, self.activations);
        }
    }

    fn get_learning_rate(&mut self) -> f64 {
        sigmoid(((self.activations - self.activation_steps) + 1).into())
    }

    fn get_output(&self) -> &Neuron {
        &self.output
    }
}
extern crate rayon;

pub mod functions;

use super::synapse::Synapse;
use functions::Functions;
use rayon::prelude::*;

pub struct Neuron {
    pub function: Functions,
    pub synapses: Vec<Synapse>,
    pub resistance: f64
}

impl Neuron {

    pub fn execute(&mut self, activation: f64, weight: f64) -> f64 {
        let resist_value: f64 = 0.0;
        if self.resistance <= activation {
            let resist_value: f64 = self.function.execute(activation, weight);
            if resist_value >= 0.0 {
                self.add_resistance(resist_value);
                self.execute_synapses(resist_value);
            }
            resist_value
        } else {
            if resist_value >= 0.0 {
                self.remove_resistance(resist_value);
            }
            resist_value
        }
    }
    
    fn add_resistance(&mut self, value: f64) {
        self.resistance += value / 10.0;
    }

    fn remove_resistance(&mut self, value: f64) {
        self.resistance -= value / 10.0;
    }

    fn execute_synapses(&mut self, value: f64) {
        self.synapses.par_iter_mut().for_each(|x| x.execute(value));
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }

    // pub fn delete_synapse(&self, &synapse: Synapse) {
    //     let index = self.synapses.par_iter().position(|x| *x == synapse);
    //     self.synapses[index].get_output().alert_deletion(&self.synapses[index]);
    //     self.synapses.remove(index);
    // }

    // pub fn alert_deletion(&self, &synapse: Synapse) {
    //     let index = self.synapses.par_iter().map(|x| *x == synapse);
    //     self.synapses.remove(index);
    // }
}
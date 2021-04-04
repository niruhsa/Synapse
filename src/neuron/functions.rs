extern crate utils;
extern crate rand;


use std::f64::consts::PI;
use utils::math::sigmoid;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub enum NeuronType {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    MULPIADD,
    SINE,
    COSINE,
    TANGENT,
    ABSOLUTE
}

pub type Functions = NeuronType;

impl Functions {
    pub fn execute(&self, a: f64, w: f64) -> f64 {
        match self {
            Self::ADDITION => sigmoid(a + w),
            Self::SUBTRACTION => sigmoid(a - w),
            Self::MULTIPLICATION => sigmoid(a * w),
            Self::DIVISION => sigmoid(a / w),
            Self::MULPIADD => sigmoid((a * PI) / w),
            Self::SINE => sigmoid(a.sin() + w.sin()),
            Self::COSINE => sigmoid(a.cos() + w.cos()),
            Self::TANGENT => sigmoid(a.tan() + w.tan()),
            Self::ABSOLUTE => sigmoid(a.abs()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::ADDITION => "ADDITION".to_string(),
            Self::SUBTRACTION => "SUBTRACTION".to_string(),
            Self::MULTIPLICATION => "MULTIPLICATION".to_string(),
            Self::DIVISION => "DIVISION".to_string(),
            Self::MULPIADD => "MULPIADD".to_string(),
            Self::SINE => "SINE".to_string(),
            Self::COSINE => "COSINE".to_string(),
            Self::TANGENT => "TANGENT".to_string(),
            Self::ABSOLUTE => "ABSOLUTE".to_string(),
        }
    }
}

impl Distribution<NeuronType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NeuronType {
        match rng.gen_range(0..8) {
            0 => NeuronType::ADDITION,
            1 => NeuronType::SUBTRACTION,
            2 => NeuronType::MULTIPLICATION,
            3 => NeuronType::DIVISION,
            4 => NeuronType::MULPIADD,
            5 => NeuronType::SINE,
            6 => NeuronType::COSINE,
            7 => NeuronType::TANGENT,
            8 => NeuronType:: ABSOLUTE,
            _ => NeuronType::ADDITION
        }
    }
}
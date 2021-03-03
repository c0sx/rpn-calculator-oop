use std::cmp::Ordering;

use super::add::Add;
use super::divide::Divide;
use super::multiply::Multiply;
use super::negative::Negative;
use super::positive::Positive;
use super::subtract::Subtract;

pub trait Op {
    fn priority(&self) -> u8;
    fn calculate(&self) -> f64;
}

pub enum Operation {
    Add { a: f64, b: f64 },
    Subtract { a: f64, b: f64 },
    Multiply { a: f64, b: f64 },
    Divide { a: f64, b: f64 },
    Negative { a: f64 },
    Positive { a: f64 },
}

impl Operation {
    fn calculate(&self) -> f64 {
        return match self {
            Operation::Add { a, b } => Add::new(*a, *b).calculate(),
            Operation::Subtract { a, b } => Subtract::new(*a, *b).calculate(),
            Operation::Multiply { a, b } => Multiply::new(*a, *b).calculate(),
            Operation::Divide { a, b } => Divide::new(*a, *b).calculate(),
            Operation::Negative { a } => Negative::new(*a).calculate(),
            Operation::Positive { a } => Positive::new(*a).calculate(),
        };
    }
}

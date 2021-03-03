use super::operation::Op;

pub struct Multiply {
    a: f64,
    b: f64,

    priority: u8,
}

impl Op for Multiply {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        self.a * self.b
    }
}

impl Multiply {
    pub fn new(a: f64, b: f64) -> Multiply {
        Multiply { a, b, priority: 2 }
    }
}

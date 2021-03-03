use super::operation::Op;

pub struct Subtract {
    a: f64,
    b: f64,

    priority: u8,
}

impl Op for Subtract {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        self.a - self.b
    }
}

impl Subtract {
    pub fn new(a: f64, b: f64) -> Subtract {
        Subtract { a, b, priority: 1 }
    }
}

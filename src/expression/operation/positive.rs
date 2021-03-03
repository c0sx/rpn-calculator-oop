use super::operation::Op;

pub struct Positive {
    a: f64,

    priority: u8,
}

impl Op for Positive {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        return if self.a < 0.0 { self.a * -1 } else { self.a };
    }
}

impl Positive {
    pub fn new(a: f64) -> Positive {
        Positive { a, priority: 3 }
    }
}

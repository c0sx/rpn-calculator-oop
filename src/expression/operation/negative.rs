use super::operation::Op;

pub struct Negative {
    a: f64,

    priority: u8,
}

impl Op for Negative {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        return if self.a > 0.0 { -self.a } else { self.a };
    }
}

impl Negative {
    pub fn new(a: f64) -> Negative {
        Negative { a, priority: 1 }
    }
}

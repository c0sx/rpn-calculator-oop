use super::operation::Op;

pub struct Add {
    a: f64,
    b: f64,

    priority: u8,
}

impl Op for Add {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        self.a + self.b
    }
}

impl Add {
    pub fn new(a: f64, b: f64) -> Add {
        Add { a, b, priority: 1 }
    }
}

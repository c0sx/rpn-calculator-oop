use super::operation::Op;

pub struct Divide {
    a: f64,
    b: f64,

    priority: u8,
}

impl Op for Divide {
    fn priority(&self) -> u8 {
        self.priority
    }

    fn calculate(&self) -> f64 {
        if self.b == 0.0 {
            panic!("division by zero");
        }

        self.a / self.b
    }
}

impl Divide {
    pub fn new(a: f64, b: f64) -> Divide {
        Divide { a, b, priority: 2 }
    }
}

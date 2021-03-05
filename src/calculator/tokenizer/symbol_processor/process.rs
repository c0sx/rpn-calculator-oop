pub trait Process {
    fn process(&self, c: char, token: &mut String) -> bool;
}

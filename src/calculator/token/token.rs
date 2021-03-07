pub trait Token {
    fn value(&self) -> String;
    fn len(&self) -> usize;
    fn priority(&self) -> Result<u8, &'static str>;
}



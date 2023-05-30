use std::iter::Cycle;


pub trait Driver {
    fn feed(&mut self, b: u8) -> Option<u8>;
}

#[derive(Clone)]
pub struct XOR<I>(Cycle<I>);

impl<I: Iterator<Item = u8> + Clone> XOR<I> {
    pub fn new(i: I) -> Self {
        Self(i.cycle())
    }
}

impl<I: Iterator<Item = u8> + Clone> Driver for XOR<I> {
    fn feed(&mut self, b: u8) -> Option<u8> {
        let key_b = self.0.next().expect("XOREncDec iterator must be infinit.");
        Some(b ^ key_b)
    }
}

#[derive(Clone)]
pub struct Addict<I>(Cycle<I>);

impl<I: Iterator<Item = u8> + Clone> Addict<I> {
    pub fn new(i: I) -> Self {
        Self(i.cycle())
    }
}

impl<I: Iterator<Item = u8> + Clone> Driver for Addict<I> {
    fn feed(&mut self, b: u8) -> Option<u8> {
        let key_b = self.0.next().expect("Addict iterator must be infinit.");
        Some(b.wrapping_add(key_b))
    }
}

#[derive(Clone)]
pub struct Subtract<I>(Cycle<I>);

impl<I: Iterator<Item = u8> + Clone> Subtract<I> {
    pub fn new(i: I) -> Self {
        Self(i.cycle())
    }
}

impl<I: Iterator<Item = u8> + Clone> Driver for Subtract<I> {
    fn feed(&mut self, b: u8) -> Option<u8> {
        let key_b = self.0.next().expect("Addict iterator must be infinit.");
        Some(b.wrapping_sub(key_b))
    }
}

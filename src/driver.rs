use std::{iter::Cycle, str::Bytes};

struct Key<'a>(Cycle<Bytes<'a>>);

impl<'a> Key<'a> {
    fn new(s: &'a str) -> Self {
        Self(s.bytes().cycle())
    }

    fn next(&mut self) -> u8 {
        self.0.next().expect("Iterator must be infinit.")
    }
}

pub trait Driver {
    fn feed(&mut self, data: &mut [u8]);
}

pub struct Basic<'a, F> {
    key: Key<'a>,
    func: F,
}

impl<'a, F> Basic<'a, F> {
    pub fn new(key: &'a str, f: F) -> Self {
        Self {
            key: Key::new(key),
            func: f,
        }
    }
}

impl<'a, F: Fn(u8, u8) -> u8> Driver for Basic<'a, F> {
    fn feed(&mut self, b: &mut [u8]) {
        for c in b {
            let key_c = self.key.next();
            *c = (self.func)(key_c, *c);
        }
    }
}

pub fn xor(key: &str) -> Basic<impl Fn(u8, u8) -> u8> {
    Basic::new(key, |k, b| k ^ b)
}

pub fn addict(key: &str) -> Basic<impl Fn(u8, u8) -> u8> {
    Basic::new(key, |k, b: u8| b.wrapping_add(k))
}

pub fn subtract(key: &str) -> Basic<impl Fn(u8, u8) -> u8> {
    Basic::new(key, |k, b: u8| b.wrapping_sub(k))
}

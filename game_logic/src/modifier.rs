use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Modifier(i32);

impl Modifier {
    pub fn new(value: i32) -> Modifier {
        Modifier(value)
    }

    pub fn zero() -> Modifier {
        Modifier(0)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn sign_char(&self) -> char {
        match self.value() {
            v if v >= 0 => '+',
            _ => '-',
        }
    }

    pub fn abs_value(&self) -> i32 {
        self.value().abs()
    }
}

impl Add<Modifier> for i32 {
    type Output = i32;

    fn add(self, rhs: Modifier) -> i32 {
        self + rhs.value()
    }
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.sign_char(), self.abs_value())
    }
}

impl std::fmt::Debug for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.sign_char(), self.abs_value())
    }
}

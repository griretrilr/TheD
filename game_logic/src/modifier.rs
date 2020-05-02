use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::Add;

/// A positive or negative modifier for a value such as a dice roll.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Modifier(i32);

impl Modifier {
    /// New modifier with the given value.
    pub fn new(value: i32) -> Modifier {
        Modifier(value)
    }

    /// The zero modifier.
    pub fn zero() -> Modifier {
        Modifier(0)
    }

    /// The signed value of the modifier.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// '-' for negative modifiers, '+' otherwise.
    pub fn sign_char(&self) -> char {
        match self.value() {
            v if v >= 0 => '+',
            _ => '-',
        }
    }

    /// The absolute value of the modifier.
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

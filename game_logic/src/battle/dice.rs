extern crate rand;

use crate::{battle::roll::Roll, modifier::Modifier, simple_rng::SimpleRng};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dice {
    size: i32,
    modifier: Modifier,
}

impl Dice {
    pub fn new(size: i32, modifier: Modifier) -> Dice {
        assert!(size >= 1);

        Dice { size, modifier }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn modifier(&self) -> Modifier {
        self.modifier
    }

    pub fn min(self) -> Roll {
        Roll::new(self, 1)
    }

    pub fn max(&self) -> Roll {
        Roll::new(*self, self.size())
    }

    pub fn roll<R: SimpleRng + ?Sized>(&self, rng: &mut R) -> Roll {
        Roll::new(*self, rng.gen_range_i32(1, self.size() + 1))
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", self.size())?;
        if self.modifier() != Modifier::zero() {
            write!(f, "{}", self.modifier())?;
        }
        Ok(())
    }
}

impl Debug for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use super::*;
    use rstest::rstest;

    #[rstest(expected_string, size, modifier_value, expected_min_total_value, expected_max_total_value,
        case("d6", 6, 0, 1, 6),
        case("d6+1", 6, 1, 2, 7),
        case("d6-1", 6, -1, 0, 5),
        case("d6+42", 6, 42, 43, 48),
        case("d6-42", 6, -42, -41, -36),
    )]
    fn test(
        size: i32,
        modifier_value: i32,
        expected_string: &str,
        expected_min_total_value: i32,
        expected_max_total_value: i32,
    ) {
        let modifier = Modifier::new(modifier_value);
        let dice = Dice::new(size, modifier);

        assert_eq!(dice.size(), size);
        assert_eq!(dice.modifier(), modifier);
        assert_eq!(dice.to_string(), expected_string);
        assert_eq!(dice.min().total_value(), expected_min_total_value);
        assert_eq!(dice.max().total_value(), expected_max_total_value);

        // The next part of the test is based on some knowledge of the implementation.
        // It verifies our implementation produces the right result given a valid SimpleRng
        // but it will probably fail if our implementation changes.

        struct MockRng {
            next: i32,
            size: i32,
        }

        impl SimpleRng for MockRng {
            fn gen_range_i32(&mut self, low: i32, high: i32) -> i32 {
                assert_eq!(low, 1);
                assert_eq!(high, self.size + 1);
                assert!(self.next <= self.size);
                let r = self.next;
                self.next += 1;
                r
            }
        }

        let mut rng = MockRng { next: 1, size };
        for face_value in 1..=size {
            let roll = dice.roll(&mut rng);
            assert_eq!(roll.face_value(), face_value);
            assert_eq!(roll.dice(), &dice);
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_asserts() {
        Dice::new(0, Modifier::zero());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn negative_size_asserts() {
        Dice::new(-1, Modifier::zero());
    }
}

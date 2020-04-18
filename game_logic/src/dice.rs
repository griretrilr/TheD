extern crate rand;

use rand::Rng;
use std::fmt;

pub struct Dice {
    size: i32,
    modifier: i32,
}

impl Dice {
    pub fn new(size: i32, modifier: i32) -> Dice {
        assert!(size >= 1);

        Dice { size, modifier }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn modifier(&self) -> i32 {
        self.modifier
    }

    pub fn min(&self) -> i32 {
        1 + self.modifier()
    }

    pub fn max(&self) -> i32 {
        self.size() + self.modifier()
    }

    pub fn roll<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
        rng.gen_range(self.min(), self.max() + 1)
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifier() > 0 {
            write!(f, "d{:?}+{:?}", self.size(), self.modifier())
        } else if self.modifier() < 0 {
            write!(f, "d{:?}{:?}", self.size(), self.modifier())
        } else {
            write!(f, "d{:?}", self.size())
        }
    }
}

impl fmt::Debug for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn new_rng() -> SmallRng {
        SeedableRng::seed_from_u64(0)
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Dice::new(1, 0).to_string(), "d1");
        assert_eq!(Dice::new(1, 1).to_string(), "d1+1");
        assert_eq!(Dice::new(1, 42).to_string(), "d1+42");
        assert_eq!(Dice::new(1, -1).to_string(), "d1-1");
        assert_eq!(Dice::new(1, -42).to_string(), "d1-42");
    }

    fn test_roll_values_case(
        size: i32,
        modifier: i32,
        expected_min: i32,
        expected_max: i32,
        rolls: i32,
    ) {
        let dice = Dice::new(size, modifier);
        let mut rng = new_rng();
        let mut min: i32 = std::i32::MAX;
        let mut max: i32 = std::i32::MIN;

        for _ in 1..rolls {
            let v = dice.roll(&mut rng);
            min = std::cmp::min(v, min);
            max = std::cmp::max(v, max);
        }

        assert_eq!(min, expected_min);
        assert_eq!(max, expected_max);
    }

    #[test]
    fn test_roll_values() {
        const ROLLS: i32 = 100;
        test_roll_values_case(1, 0, 1, 1, ROLLS);
        test_roll_values_case(1, 10, 11, 11, ROLLS);
        test_roll_values_case(20, 0, 1, 20, ROLLS);
        test_roll_values_case(20, 3, 4, 23, ROLLS);
        test_roll_values_case(20, -3, -2, 17, ROLLS);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_asserts() {
        Dice::new(0, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn negative_size_asserts() {
        Dice::new(-1, 0);
    }
}

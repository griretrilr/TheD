extern crate rand;

use rand::Rng;

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

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn new_rng() -> SmallRng {
        SeedableRng::seed_from_u64(0)
    }

    fn test_case(size: i32, modifier: i32, expected_min: i32, expected_max: i32, rolls: i32) {
        let dice = Dice::new(size, modifier);
        let mut rng = new_rng();
        for _ in 1..rolls {
            let v = dice.roll(&mut rng);
            assert!(v >= expected_min);
            assert!(v <= expected_max);
        }
    }

    #[test]
    fn test() {
        const ROLLS: i32 = 100;
        test_case(1, 0, 1, 1, ROLLS);
        test_case(1, 10, 11, 11, ROLLS);
        test_case(20, 3, 4, 23, ROLLS);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_asserts() {
        Dice::new(0, 0);
    }
}

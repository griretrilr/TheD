use super::{Dice, Modifier};

#[derive(Copy, Clone)]
pub struct Roll {
    dice: Dice,
    face_value: i32,
}

impl Roll {
    pub fn new(dice: Dice, face_value: i32) -> Roll {
        assert!(face_value >= 1 && face_value <= dice.size());
        Roll { dice, face_value }
    }

    pub fn dice(&self) -> &Dice {
        &self.dice
    }

    pub fn face_value(&self) -> i32 {
        self.face_value
    }

    pub fn modifier(&self) -> Modifier {
        self.dice().modifier()
    }

    pub fn total_value(&self) -> i32 {
        self.face_value() + self.dice().modifier()
    }
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} {} {})",
            self.total_value(),
            self.face_value(),
            self.modifier().sign_char(),
            self.modifier().abs_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Dice, Modifier, Roll};

    #[test]
    fn test() {
        struct TestCase<'a> {
            dice_size: i32,
            dice_modifier: i32,
            face_value: i32,
            expected_total_value: i32,
            expected_string: &'a str,
        }

        impl TestCase<'_> {
            fn run(&self) {
                let modifier = Modifier::new(self.dice_modifier);
                let roll = Roll::new(Dice::new(self.dice_size, modifier), self.face_value);
                assert_eq!(roll.face_value(), self.face_value);
                assert_eq!(roll.modifier(), modifier);
                assert_eq!(roll.total_value(), self.expected_total_value);
                assert_eq!(roll.to_string(), self.expected_string);
            }
        }

        TestCase {
            dice_size: 6,
            dice_modifier: 0,
            face_value: 3,
            expected_total_value: 3,
            expected_string: "3 (3 + 0)",
        }
        .run()
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_face_value_asserts() {
        let _ = Roll::new(Dice::new(6, Modifier::zero()), 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn negative_face_value_asserts() {
        let _ = Roll::new(Dice::new(6, Modifier::zero()), -1);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn face_value_too_high_asserts() {
        let _ = Roll::new(Dice::new(6, Modifier::zero()), 7);
    }
}

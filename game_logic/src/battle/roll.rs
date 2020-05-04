use crate::battle::dice::Dice;
use crate::modifier::Modifier;

#[derive(Copy, Clone)]
pub struct Roll<'a> {
    dice: &'a Dice,
    face_value: i32,
}

impl Roll<'_> {
    pub fn new<'a>(dice: &'a Dice, face_value: i32) -> Roll<'a> {
        assert!(face_value >= 1 && face_value <= dice.size());
        Roll { dice, face_value }
    }

    pub fn dice(&self) -> &Dice {
        self.dice
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

impl std::fmt::Display for Roll<'_> {
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
    extern crate rstest;

    use super::{Dice, Modifier, Roll};
    use rstest::rstest;

    #[rstest(dice_size, dice_modifier, face_value, expected_total_value, expected_string, 
        case(6, 0, 1, 1, "1 (1 + 0)"),
        case(6, 1, 3, 4, "4 (3 + 1)"), 
        case(6, 42, 6, 48, "48 (6 + 42)"),
        case(20, -1, 10, 9, "9 (10 - 1)"),
        case(1, -42, 1, -41, "-41 (1 - 42)"))]
    fn test(dice_size: i32, dice_modifier: i32, face_value: i32, expected_total_value: i32, expected_string: &str) {
        let modifier = Modifier::new(dice_modifier);
        let dice = Dice::new(dice_size, modifier);
        let roll = Roll::new(&dice, face_value);
        
        assert_eq!(roll.face_value(), face_value);
        assert_eq!(roll.modifier(), modifier);
        assert_eq!(roll.total_value(), expected_total_value);
        assert_eq!(roll.to_string(), expected_string);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_face_value_asserts() {
        let _ = Roll::new(&Dice::new(6, Modifier::zero()), 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn negative_face_value_asserts() {
        let _ = Roll::new(&Dice::new(6, Modifier::zero()), -1);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn face_value_too_high_asserts() {
        let _ = Roll::new(&Dice::new(6, Modifier::zero()), 7);
    }
}

extern crate itertools;

use crate::battle::{DiceSet, Roll};
use itertools::Itertools;

/// A set of dice rolled in an attack.
pub struct RollSet<'a> {
    dice: &'a DiceSet,
    rolls: Vec<Roll<'a>>,
    sorted_rolls: Vec<usize>,
}

impl RollSet<'_> {
    pub fn new<'a>(dice: &'a DiceSet, rolls: Vec<Roll<'a>>) -> RollSet<'a> {
        let sorted_rolls: Vec<usize> = (0..rolls.len())
            .sorted_by_key(|i| -rolls[*i].total_value())
            .collect();
        RollSet {
            dice,
            rolls,
            sorted_rolls,
        }
    }

    pub fn dice(&self) -> &DiceSet {
        self.dice
    }

    pub fn rolls(&self) -> &Vec<Roll> {
        &self.rolls
    }

    pub fn len(&self) -> usize {
        self.dice().len()
    }

    pub fn sorted_rolls(&self) -> SortedRolls {
        SortedRolls { roll_set: &self }
    }
}

impl std::fmt::Display for RollSet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, r) in self.sorted_rolls() {
            write!(f, "{} ({}/{}) = {}", self.dice()[i], i + 1, self.len(), r)?;
        }
        Ok(())
    }
}

pub struct SortedRolls<'a> {
    roll_set: &'a RollSet<'a>,
}

impl SortedRolls<'_> {
    pub fn len(&self) -> usize {
        self.roll_set.len()
    }
}

impl<'a> IntoIterator for SortedRolls<'a> {
    type Item = SortedRoll<'a>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(&self) -> Self::IntoIter {
       Box::new((0..self.len()).map(|i| SortedRoll<'a>{ roll_set: self.roll_set, original_index: i }))
    }
}

pub struct SortedRoll<'a> {
    roll_set: &'a RollSet<'a>,
    original_index: usize,
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use crate::Modifier;
    use crate::battle::{Dice, Roll};
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

extern crate itertools;

use crate::battle::{DiceSet, Roll};
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;

/// The result of rolling a collection of dice.
pub struct RollSet<'a> {
    dice_set: &'a DiceSet<'a>,
    raw_rolls: Vec<Roll<'a>>,
    sort_order: Vec<usize>,
}

/// Information about one roll in a roll set.
pub struct RollSetRoll<'a> {
    roll_set: &'a RollSet<'a>,
    index_in_dice_set: usize,
    sorted_index: usize,
}

/// The rolls in a roll set, in the same order as the dice in the dice set.
pub struct RollSetRawRolls<'a> {
    roll_set: &'a RollSet<'a>,
}

/// The rolls in a roll set, sorted from highest face value to lowest.
pub struct RollSetSortedRolls<'a> {
    roll_set: &'a RollSet<'a>,
}

impl RollSet<'_> {
    /// Creates a roll set from a dice set and a vector of one roll for each dice in the set.
    pub fn new<'a>(dice_set: &'a DiceSet, rolls: Vec<Roll<'a>>) -> RollSet<'a> {
        for i in 0..rolls.len() {
            assert_eq!(rolls[i].dice(), dice_set[i]);
        }
        let sort_order: Vec<usize> = (0..rolls.len()).sorted_by_key(|i| rolls[*i].total_value()).collect();
        RollSet {
            dice_set,
            raw_rolls: rolls,
            sort_order,
        }
    }

    pub fn len(&self) -> usize {
        self.dice_set.len()
    }

    pub fn sorted_rolls(&self) -> RollSetSortedRolls {
        RollSetSortedRolls { roll_set: self }
    }

    pub fn raw_rolls(&self) -> RollSetRawRolls {
        RollSetRawRolls { roll_set: self }
    }
}

impl Display for RollSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for &d in self.sorted_rolls() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", d)?;
            first = false;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Debug for RollSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<'a> Index<usize> for RollSetSortedRolls<'a> {
    type Output = RollSetRoll<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        let original_index = self.roll_set.sort_order[index];
        RollSetRoll { index_in_dice_set: original_index, sorted_index: index }
    }
}

#[cfg(test)]
mod tests {
    use crate::Modifier;
    use crate::battle::{Dice, DiceSet};
    use rstest::rstest;

    #[rstest(expected_string, dice,
        case("{d6}", vec!(Dice::new(6, Modifier::zero()))),
        case("{d6+1, d10-3}", vec!(Dice::new(6, Modifier::new(1)), Dice::new(10, Modifier::new(-3)))),
    )]
    fn test_formatting(
        expected_string: &str,
        dice: Vec<Dice>,
    ) {
        let dice_set = DiceSet::new(dice.iter().collect());
        let display = format!("{}", dice_set);
        let debug = format!("{:?}", dice_set);
        assert_eq!(display, expected_string);
        assert_eq!(debug, expected_string);
    }

    /// Ensures that the dice set dereferences as the passed-in vector of dice references.
    #[rstest(dice,
        case(vec!(Dice::new(6, Modifier::zero()))),
        case(vec!(Dice::new(6, Modifier::new(1)), Dice::new(10, Modifier::new(-3)))),
    )]
    fn test_acts_like_vec_of_dice_refs(
        dice: Vec<Dice>,
    ) {
        let dice_refs_in: Vec<&Dice> = dice.iter().collect();
        let dice_set = DiceSet::new(dice_refs_in.clone());
        let dice_refs_out = (*dice_set).clone();
        assert_eq!(dice_refs_in, dice_refs_out);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn new_with_empty_dice_asserts() {
        DiceSet::new(Vec::new());
    }
}

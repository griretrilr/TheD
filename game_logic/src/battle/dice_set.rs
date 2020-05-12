use crate::battle::Dice;
use std::fmt::{Debug, Display, Formatter};

/// A collection of dice rolled in battle.
pub struct DiceSet<'a> {
    dice: Vec<&'a Dice>,
}

impl DiceSet<'_> {
    pub fn new(dice: Vec<&Dice>) -> DiceSet {
        assert!(dice.len() > 0);

        DiceSet { dice }
    }
}

impl<'a> std::ops::Deref for DiceSet<'a> {
    type Target = Vec<&'a Dice>;

    fn deref(&self) -> &Self::Target {
        &self.dice
    }
}

impl Display for DiceSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for &d in self.iter() {
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

impl Debug for DiceSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
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

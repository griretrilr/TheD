use crate::battle::{Dice, Roll, RollSet};
use crate::SimpleRng;

/// A set of dice to be rolled in an attack.
pub struct DiceSet {
    dice: Vec<Dice>,
}

impl DiceSet {
    pub fn new(dice: Vec<Dice>) -> DiceSet {
        DiceSet { dice }
    }

    pub fn dice(&self) -> &Vec<Dice> {
        &self.dice
    }

    pub fn len(&self) -> usize {
        self.dice().len()
    }

    pub fn roll<R: SimpleRng + ?Sized>(&self, rng: &mut R) -> RollSet {
        let rolls: Vec<Roll> = (0..self.len()).map(|i| self.dice()[i].roll(rng)).collect();

        RollSet::new(&self, rolls)
    }
}

impl std::ops::Index<usize> for DiceSet {
    type Output = Dice;

    fn index(&self, i: usize) -> &Self::Output {
        &self.dice()[i]
    }
}

impl std::iter::IntoIterator for DiceSet {
    type Item = Dice;
    type IntoIter = std::vec::IntoIter<Dice>;

    fn into_iter(self) -> Self::IntoIter {
        self.dice.into_iter()
    }
}

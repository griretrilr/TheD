mod dice;
mod game;
mod modifier;
mod roll;
mod simple_rng;

pub use dice::Dice;
pub use game::Game;
pub use modifier::Modifier;
pub use roll::Roll;
pub use simple_rng::SimpleRng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = Game::new();
        assert_eq!(g.magic, 42);
    }
}

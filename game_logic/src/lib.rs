mod game;

pub use game::Game;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = Game::new();
        assert_eq!(g.magic, 42);
    }
}

extern crate game_logic;
extern crate pancurses;

use pancurses::{endwin, initscr, noecho, resize_term, Input};

const WINDOW_WIDTH: i32 = 100;
const WINDOW_HEIGHT: i32 = 30;

fn main() {
    let window = initscr();
    resize_term(WINDOW_HEIGHT, WINDOW_WIDTH);
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                window.addch(c);
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
        window.refresh();
    }
    endwin();
}

extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input};

fn main() {
    let window = initscr();
    window.printw("Type things, press delete to quit\n");
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
    }
    endwin();
}

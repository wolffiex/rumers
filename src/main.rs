extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, cbreak};
use std::time::SystemTime;

mod font;

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Starting(usize),
    Running(SystemTime),
    Paused(SystemTime, SystemTime),
}


fn main() {
    let window = initscr();
    window.keypad(true);
    noecho();
    cbreak();
    window.printw("Hello Rust");
    window.refresh();
    window.timeout(250);
    let mut state = State::Starting(0);
    let mut i = 0;
    while render(state) {
        state = match window.getch() {
            Some(input) => match state {
                State::Starting(minutes) => adjust_timer(minutes, input),
                State::Running(_) => state,
                State::Paused(_, _) => state,
            }
            None => state
        }
    };
    endwin();
}

fn adjust_timer(minutes: usize, _input: Input) -> State {
    State::Starting(minutes)
}

fn render(state: State) -> bool {
    true
}
extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, cbreak, Window};
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
    let f = font::get_font();
    println!("font is {:#?}", f);
    window.keypad(true);
    noecho();
    cbreak();
    window.printw("Hello Rust");
    window.refresh();
    window.timeout(250);
    let mut state = State::Starting(0);
    while render(&window, state) {
        state = match window.getch() {
            Some(input) => match state {
                State::Starting(minutes) => adjust_timer(minutes, input),
                State::Running(end_time) => while_running(end_time, input),
                State::Paused(end_time, pause_time) =>
                    while_paused(end_time, pause_time, input)
            }
            None => state
        }
    };
    endwin();
}

fn while_running(end_time: SystemTime, input: Input) -> State {
    State::Running(end_time)
}

fn while_paused(end_time: SystemTime, paused_time: SystemTime, input: Input) -> State {
    State::Paused(end_time, paused_time)
}

fn adjust_timer(minutes: usize, input: Input) -> State {
    match input {
        Input::KeyEnter => State::Running(SystemTime::now()),
        Input::KeyUp => State::Starting(minutes + 1),
        Input::KeyDown => State::Starting(minutes - 1),
        _ => State::Starting(minutes),
    }
}

fn render(window: &Window, state: State) -> bool {
    true
}
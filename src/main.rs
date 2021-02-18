extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, cbreak, Window};
use std::time::{SystemTime, Duration};
use std::cmp;
use std::alloc::System;

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
                State::Starting(minutes) => setup_mode(minutes, input),
                State::Running(end_time) => run_mode(end_time, input),
                State::Paused(end_time, pause_time) =>
                    pause_mode(end_time, pause_time, input)
            }
            None => state
        }
    };
    endwin();
}

fn setup_mode(minutes: usize, input: Input) -> State {
    match input {
        Input::Character(' ') | Input::KeyEnter => run_state(minutes),
        Input::KeyUp => State::Starting(minutes + 1),
        Input::KeyDown => State::Starting(if minutes > 0 { minutes - 1 } else { 0 }),
        _ => State::Starting(minutes),
    }
}

fn run_state(minutes: usize) -> State {
    State::Running(
        SystemTime::now().checked_add(
            Duration::from_secs((minutes * 60) as u64)).unwrap())
}


fn run_mode(end_time: SystemTime, input: Input) -> State {
    match input {
        Input::Character(' ') => State::Paused(end_time, SystemTime::now()),
        _ => State::Running(end_time)
    }
}

fn pause_mode(end_time: SystemTime, paused_time: SystemTime, input: Input) -> State {
    match input {
        Input::Character(' ') => State::Running(SystemTime::now()),
        _ => State::Paused(end_time, paused_time)
    }
}

fn render(window: &Window, state: State) -> bool {
    let (minutes, seconds) = match state {
        State::Starting(minutes) => (minutes, 0 as usize),
        State::Running(end_time) => min_sec_until(end_time),
        State::Paused(end_time, pause_time) => (13, 14),
    };
    window.clear();
    window.printw(format!("{}:{}", minutes, seconds));
    true
}

fn min_sec_until(end_time: SystemTime) -> (usize, usize) {
    let duration = end_time.duration_since(SystemTime::now()).unwrap();
    let secs = duration.as_secs() as usize;
    (secs / 10, secs % 60)
}
extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, cbreak, Window};
use std::time::{Duration, Instant};

mod font;

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Starting(usize),
    Running(Instant),
    Paused(Instant, Instant),
}


fn main() {
    let f = font::get_font();
    return ();
    let window = initscr();
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
        Input::Character(' ') | Input::Character('\n') | Input::KeyEnter => run_state(minutes),
        Input::KeyUp => State::Starting(minutes + 1),
        Input::KeyDown => State::Starting(if minutes > 0 { minutes - 1 } else { 0 }),
        _ => State::Starting(minutes),
    }
}

fn run_state(minutes: usize) -> State {
    State::Running(Instant::now() + Duration::from_secs((minutes * 60) as u64))
}


fn run_mode(end_time: Instant, input: Input) -> State {
    match input {
        Input::Character(' ') => State::Paused(end_time, Instant::now()),
        _ => State::Running(end_time)
    }
}

fn pause_mode(end_time: Instant, paused_time: Instant, input: Input) -> State {
    match input {
        Input::Character(' ') => State::Running(Instant::now()),
        _ => State::Paused(end_time, paused_time)
    }
}

fn render(window: &Window, state: State) -> bool {
    let (minutes, seconds) = match state {
        State::Starting(minutes) => (minutes, 0 as usize),
        State::Running(end_time) => min_sec_until(Instant::now(), end_time),
        State::Paused(end_time, pause_time) =>
            min_sec_until(pause_time, end_time)
    };
    window.clear();
    window.printw(format!("{}:{}", minutes, seconds));
    true
}

fn min_sec_until(from_time: Instant, to_time: Instant) -> (usize, usize) {
    let duration = to_time.saturating_duration_since(from_time);
    let secs = duration.as_secs() as usize;
    (secs / 60, secs % 60)
}
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
    let font = font::get_font();
    let window = initscr();
    window.keypad(true);
    noecho();
    cbreak();
    window.printw("Hello Rust");
    window.refresh();
    window.timeout(250);
    let mut state = State::Starting(0);
    while render(&window, state, &font) {
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

fn render(window: &Window, state: State, font: &Vec<String>) -> bool {
    let (minutes, seconds) = match state {
        State::Starting(minutes) => (minutes, 0 as usize),
        State::Running(end_time) => min_sec_until(Instant::now(), end_time),
        State::Paused(end_time, pause_time) =>
            min_sec_until(pause_time, end_time)
    };
    window.clear();
    let m_tens = minutes/10;
    let m_ones = minutes%10;
    //window.printw(format!("{}:{}", minutes, seconds));
    render_numeral(window, 2, 2, &font[m_tens]);
    render_numeral(window, 12, 2, &font[m_ones]);
    true
}

fn render_numeral(window: &Window, x: usize, y: usize, numeral: &str) {
    let mut offset:i32 = 0;
    for line in numeral.lines() {
        window.mvaddstr(y as i32 + offset, x as i32, line);
        offset = offset + 1;
    }

}

fn min_sec_until(from_time: Instant, to_time: Instant) -> (usize, usize) {
    let duration = to_time.saturating_duration_since(from_time);
    let secs = duration.as_secs() as usize;
    (secs / 60, secs % 60)
}
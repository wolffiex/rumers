extern crate pancurses;

use pancurses::{Input, Window, COLOR_PAIR, COLOR_BLACK};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use rodio::{Source};

mod font;

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Starting(usize),
    Running(Instant),
    Paused(Instant, Instant),
}

fn start_pancurses() -> Window {
    let window = pancurses::initscr();
    pancurses::start_color();
    pancurses::init_color(10, 300, 300, 300);
    pancurses::init_color(11, 700, 700, 300);
    pancurses::init_pair(1, 10, COLOR_BLACK);
    pancurses::init_pair(2, 11, COLOR_BLACK);
    pancurses::noecho();
    pancurses::cbreak();
    pancurses::curs_set(0);
    window.keypad(true);
    window
}

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("src/alarm.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let font = font::get_font();
    let window = start_pancurses();
    window.timeout(100);
    let start_time: Instant = Instant::now();
    let mut is_done = false;
    let mut state = State::Starting(0);
    while !is_done {
        let current_time = Instant::now();
        render(&window, state, &font, (start_time, current_time));
        state = match window.getch() {
            Some(input) => match state {
                State::Starting(minutes) => setup_mode(minutes, input, current_time),
                State::Running(end_time) => run_mode(end_time, input, current_time),
                State::Paused(end_time, pause_time) =>
                    pause_mode(end_time, pause_time, input, current_time),
                _ => state,
            },
            _ => {state}
        };
        if let State::Running(end_time) = state {
            is_done = end_time.saturating_duration_since(current_time).as_millis() == 0;
        }
    };
    stream_handle.play_raw(source.convert_samples()).unwrap();
    pancurses::endwin();
}

fn setup_mode(minutes: usize, input: Input, current_time: Instant) -> State {
    match input {
        Input::Character(' ') | Input::Character('\n') | Input::KeyEnter => run_state(minutes, current_time),
        Input::KeyUp => State::Starting(minutes + 1),
        Input::KeyDown => State::Starting(if minutes > 0 { minutes - 1 } else { 0 }),
        _ => State::Starting(minutes),
    }
}

fn run_state(minutes: usize, current_time: Instant) -> State {
    State::Running(current_time + Duration::from_secs(10))//(minutes * 60) as u64))
}


fn run_mode(end_time: Instant, input: Input, current_time: Instant) -> State {
    match input {
        Input::Character(' ') => State::Paused(end_time, current_time),
        _ => State::Running(end_time)
    }
}

fn pause_mode(end_time: Instant, paused_time: Instant, input: Input, current_time: Instant) -> State {
    match input {
        Input::Character(' ') => {
            let remaining = end_time - paused_time;
            State::Running(current_time + remaining)
        }
        _ => State::Paused(end_time, paused_time)
    }
}

fn render(window: &Window, state: State, font: &Vec<String>, (start_time, time_now): (Instant, Instant)) -> bool {
    let (minutes, seconds) = match state {
        State::Starting(minutes) => (minutes, 0 as usize),
        State::Running(end_time) => min_sec_until(time_now, end_time),
        State::Paused(end_time, pause_time) =>
            min_sec_until(pause_time, end_time),
    };
    window.clear();
    let m_tens = minutes / 10;
    let m_ones = minutes % 10;
    let s_tens = seconds / 10;
    let s_ones = seconds % 10;
    const TOP: usize = 2;
    let time_color = match state {
        State::Paused(_, _) =>
            if ((time_now - start_time).as_millis() / 800) % 2 == 1 { 10 } else { 0 }
        _ => 0
    };
    window.attrset(COLOR_PAIR(time_color));
    if m_tens > 0 { render_numeral(window, 2, TOP, &font[m_tens]) }
    render_numeral(window, 12, TOP, &font[m_ones]);
    render_numeral(window, 24, TOP, &font[s_tens]);
    render_numeral(window, 34, TOP, &font[s_ones]);
    if let State::Running(_) = state {
        let duration = time_now - start_time;
        if (duration.as_millis() / 800) % 2 == 0 {
            window.attrset(COLOR_PAIR(1));
        }
    } else {
        window.attrset(COLOR_PAIR(0));
    }
    for &y in [4, 6].iter() {
        window.mvaddstr(y, 22, r"x");
    }
    window.refresh();
    true
}

fn render_numeral(window: &Window, x: usize, y: usize, numeral: &str) {
    let mut offset: i32 = 0;
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
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
        state = match window.getch() {
            Some(input) => match state {
                State::Starting(minutes) => setup_mode(minutes, input, current_time),
                State::Running(end_time) => run_mode(end_time, input, current_time),
                State::Paused(end_time, pause_time) =>
                    pause_mode(end_time, pause_time, input, current_time),
            },
            _ => { state }
        };
        let (minutes, seconds) = match state {
            State::Starting(minutes) => (minutes, 0 as usize),
            State::Running(end_time) => min_sec_until(current_time, end_time),
            State::Paused(end_time, pause_time) =>
                min_sec_until(pause_time, end_time),
        };
        let digits = [minutes / 10, minutes % 10, seconds / 10, seconds % 10];
        let is_blink_on = ((current_time - start_time).as_millis() / 800) % 2 == 1;

        let colors = if is_blink_on {
            match state {
                State::Running(..) => (0, 1),
                State::Paused(..) => (0, 0),
                _ => (0, 0),
            }
        } else {
            (0, 0)
        };

        render(&window, &font, digits, colors);

        if let State::Running(end_time) = state {
            is_done = end_time.saturating_duration_since(current_time).as_millis() == 0;
        }
    };
    let done_time = Instant::now();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let mut is_blink_on = true;
    let sleep_duration = Duration::from_millis(800);
    while Instant::now().duration_since(done_time).as_secs() < 10 {
        let colors = if is_blink_on { (1, 1) } else { (0, 0) };
        render(&window, &font, [0, 0, 0, 0], colors);
        std::thread::sleep(sleep_duration);
        is_blink_on = !is_blink_on;
    }
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
    //State::Running(current_time + Duration::from_secs((minutes * 60) as u64))
    State::Running(current_time + Duration::from_secs(2)) //(minutes * 60) as u64))
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

fn render(window: &Window, font: &Vec<String>, digits: [usize; 4],
          (time_color, separator_color): (u32, u32)) {
    window.clear();
    const TOP: usize = 2;

    window.attrset(COLOR_PAIR(time_color));
    if digits[0] > 0 { render_numeral(window, 2, TOP, &font[digits[0]]) }
    render_numeral(window, 12, TOP, &font[digits[1]]);
    render_numeral(window, 24, TOP, &font[digits[2]]);
    render_numeral(window, 34, TOP, &font[digits[3]]);

    window.attrset(COLOR_PAIR(separator_color));
    for &y in [4, 6].iter() {
        window.mvaddstr(y, 22, r"x");
    }
    window.refresh();
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
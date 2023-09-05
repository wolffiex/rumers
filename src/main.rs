extern crate pancurses;

use pancurses::{Input, Window, COLOR_PAIR, COLOR_BLACK};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    minutes: Option<u8>,
}


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
    pancurses::init_pair(1, 10, COLOR_BLACK);
    pancurses::noecho();
    pancurses::cbreak();
    pancurses::curs_set(0);
    window.keypad(true);
    window
}

const BLINK_MS: u128 = 800;

fn main() {
    let args = Args::parse();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("src/411090__inspectorj__wind-chime-gamelan-gong-a.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let font = font::get_font();
    let window = start_pancurses();
    window.timeout(100);
    let start_time: Instant = Instant::now();
    let mut is_done = false;
    let mut state = if let Some(minutes) = args.minutes {
        State::Running(Instant::now() + Duration::from_secs(minutes as u64 * 60))
    } else {
        State::Starting(1)
    };
    while !is_done {
        let maybe_input = window.getch();
        let current_time = Instant::now();
        if let Some(input) = maybe_input {
            state = handle_input(state, input, current_time)
        }
        let (minutes, seconds) = match state {
            State::Starting(minutes) => (minutes, 0 as usize),
            State::Running(end_time) => min_sec_until(current_time, end_time),
            State::Paused(end_time, pause_time) =>
                min_sec_until(pause_time, end_time),
        };
        let digits = [minutes / 10, minutes % 10, seconds / 10, seconds % 10];
        let is_blink_on = ((current_time - start_time).as_millis() / BLINK_MS) % 2 == 1;

        let colors = if is_blink_on {
            match state {
                State::Running(..) => (0, 1),
                State::Paused(..) => (1, 0),
                _ => (0, 0),
            }
        } else {
            (0, 0)
        };

        render(&window, &font, digits, colors);

        is_done = if let State::Running(end_time) = state {
            end_time.saturating_duration_since(current_time).as_millis() == 0
        } else {false}
    };
    let done_time = Instant::now();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let mut is_blink_on = true;
    let sleep_duration = Duration::from_millis(BLINK_MS as u64);
    while Instant::now().duration_since(done_time).as_secs() < 10 {
        let colors = if is_blink_on { (1, 1) } else { (0, 0) };
        render(&window, &font, [0, 0, 0, 0], colors);
        std::thread::sleep(sleep_duration);
        is_blink_on = !is_blink_on;
    }
    pancurses::endwin();
}

fn handle_input(state: State, input: Input, current_time: Instant) -> State {
    match state {
        State::Starting(minutes) => {
            match input {
                Input::Character(' ') | Input::Character('\n') | Input::KeyEnter =>
                    State::Running(current_time + Duration::from_secs((minutes * 60) as u64)),
                Input::KeyUp => State::Starting(minutes + 1),
                Input::KeyDown => State::Starting(if minutes > 1 { minutes - 1 } else { 1 }),
                _ => State::Starting(minutes),
            }
        },
        _ => match input {
            Input::Character(' ') | Input::Character('\n') | Input::KeyEnter => match state {
                State::Running(end_time) => State::Paused(end_time, current_time),
                State::Paused(end_time, pause_time) =>
                    State::Running(current_time + (end_time - pause_time)),
                _ => state
            },
            _ => state
        }
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
    let secs = duration.as_secs() as usize + if duration.subsec_nanos() > 0 {1} else {0};

    (secs / 60, secs % 60)
}

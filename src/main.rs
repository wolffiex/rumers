extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, cbreak};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use crate::InputState::{Ready, Quitting};
use std::time::SystemTime;

mod font;

#[derive(Debug, PartialEq)]
enum InputState {
    Ready,
    Starting(usize),
    Running(SystemTime),
    Paused(SystemTime, SystemTime),
    Quitting,
}

#[derive(Debug)]
struct State {
    timers: Vec<(f64, Option<f64>)>,
    input_state: InputState,
}

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<State>, Receiver<State>) = mpsc::channel();

    // The sender endpoint can be copied
    let thread_tx = tx.clone();

    // Each thread will send its id via the channel
    let worker = thread::spawn(move || {
        let id = 77;
        let state = State {
            timers: vec!((1.0, None)),
            input_state: Ready,
        };
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        thread_tx.send(state).unwrap();

        // Sending is a non-blocking operation, the thread will continue
        // immediately after sending its message
        println!("thread {} finished", id);
    });

    let window = initscr();
    window.keypad(true);
    noecho();
    cbreak();
    window.printw("Hello Rust");
    window.refresh();
    window.timeout(250);
    let mut input_state = Ready;
    let mut i = 0;
    while input_state != Quitting {
        let k = window.getch();
        match k {
            Some(Input::Character('q')) => { input_state = Quitting }
            Some(input) => { window.addstr(&format!("{:?}", input)); }
            None => {
                if i % 10 == 0 {
                    window.addstr(format!("i {}", i));
                    ()
                }
                i = i +1;
            }
        }
    }
    endwin();

    let view_model = rx.recv();
    println!("{:?}", view_model);
    worker.join().expect("oops! the child thread panicked");

    // Show the order in which the messages were sent
}
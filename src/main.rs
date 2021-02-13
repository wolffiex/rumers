use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use crate::InputState::Ready;

#[derive(Debug)]
enum InputState {
    Ready,
    Labeling,
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
            timers: vec!(( 1.0, None)),
            input_state: Ready,
        };
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        thread_tx.send(state).unwrap();

        // Sending is a non-blocking operation, the thread will continue
        // immediately after sending its message
        println!("thread {} finished", id);
    });

    let getch = rx.recv();
    worker.join().expect("oops! the child thread panicked");

    // Show the order in which the messages were sent
    println!("{:?}", getch);
}
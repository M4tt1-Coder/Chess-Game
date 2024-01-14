//source: https://github.com/remones/ticker-rs/blob/master/src/lib.rs

//for a thread usage example look at this: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

//using statements
use std::sync::mpsc::{channel, Receiver};
use std::thread::{self, JoinHandle};
use std::time::Duration;

//constants
const TICKER_DURATION: Duration = Duration::new(1, 0);

///
pub struct Ticker {
    pub receiver: Receiver<bool>,
    pub handle: JoinHandle<bool>,
}

impl Ticker {
    pub fn new() -> Ticker {
        let (handle, receiver) = start_ticker();
        Ticker { receiver, handle }
    }

    pub fn stop(self) {
        drop(self.receiver);
    }
}
fn start_ticker() -> (JoinHandle<bool>, Receiver<bool>) {
    let (sender, receiver) = channel();

    //TODO - change to while loop to avoid infinite loop
    let handle = thread::spawn(move || loop {
        thread::sleep(TICKER_DURATION);
        sender.send(true).unwrap();
    });

    (handle, receiver)
}

impl Iterator for &Ticker {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.receiver.recv().is_ok() {
            Some(())
        } else {
            None
        }
    }
}

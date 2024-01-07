//TODO - maybe use cow to warp the player players for access in the moved thread
//source: https://github.com/remones/ticker-rs/blob/master/src/lib.rs

//using statements
use crate::Game;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

//constants
const TICKER_DURATION: Duration = Duration::new(1, 0);

///
struct Ticker {
    receiver: Receiver<bool>,
    sender: Sender<bool>,
}

impl Ticker {
    pub fn new() -> Ticker {
        let (sender, receiver) = start_ticker();
        Ticker {
            receiver,
            sender,
        }
    }

    pub fn stop(self) {
        drop(self.sender);
    }
}
fn start_ticker() -> (Sender<bool>, Receiver<bool>) {
    let (sender, receiver) = channel();
    //TODO look if you can improve cloning attempt
    let sender_for_thread = sender.clone();

    thread::spawn(move || loop {
        let subtract_now = true;
        thread::sleep(TICKER_DURATION);
        sender_for_thread.send(subtract_now).unwrap();
    });

    (sender, receiver)
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

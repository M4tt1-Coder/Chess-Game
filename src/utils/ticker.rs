//source: https://github.com/remones/ticker-rs/blob/master/src/lib.rs

//for a thread usage example look at this: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

//using statements
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::structs::Player;
use crate::Game;

//constants
const TICKER_DURATION: Duration = Duration::new(1, 0);

///
pub struct Ticker {
    pub receiver: Receiver<bool>,
    pub handle: JoinHandle<bool>,
}

impl Ticker {
    pub fn new(game: &Game) -> Ticker {
        let (handle, receiver) = start_ticker(game.player_one.clone(), game.player_two.clone());
        Ticker { receiver, handle }
    }

    //maybe just use the 'park' function
    pub fn stop(self) {
        drop(self.receiver);
    }
}

fn start_ticker(
    player_one: Arc<Mutex<Player>>,
    player_two: Arc<Mutex<Player>>,
) -> (JoinHandle<bool>, Receiver<bool>) {
    // let (sender, receiver) = channel();
    let (_, receiver) = channel();

    let handle = thread::spawn(move || loop {
        thread::sleep(TICKER_DURATION);

        let player_one_turn: bool = match player_one.try_lock() {
            Ok(player) => player.turn,
            Err(_) => false,
        };

        let player_two_turn: bool = match player_two.try_lock() {
            Ok(player) => player.turn,
            Err(_) => false,
        };

        if player_one_turn {
            player_one.try_lock().unwrap().seconds -= 1;
        } else if player_two_turn {
            player_two.try_lock().unwrap().seconds -= 1;
        }

        //sender.send(true).unwrap();
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

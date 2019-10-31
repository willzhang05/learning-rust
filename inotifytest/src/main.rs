// William Zhang

use std::env;
use std::fs;
use inotify::{
    Inotify,
    WatchMask,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
        inotify.add_watch(&args[1], WatchMask::MODIFY | WatchMask::CLOSE).expect("Error while adding file watch");
        let mut buffer = [0; 1024];
        loop {
            let events = inotify.read_events(&mut buffer).expect("Error while reading events");
            for event in events {
                println!("{:?}", event);
            }
        }
    } else {
        eprintln!("Invalid arguments.");
    }

}

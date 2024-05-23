use core::panic;
use inotify::{EventMask, Inotify, WatchMask};
use std::{env, path::Path};

fn main() {
    if env::args().len() < 2 {
        panic!("USE: watch PATH")
    }

    let last_arg = env::args().last().unwrap();
    let base_path = Path::new(&last_arg);
    let file_name = base_path.file_name().unwrap().to_str().unwrap();

    println!("{}", file_name);

    let mut inotify = Inotify::init().expect("Failed to create Inotify");
    let watch_mask = WatchMask::CREATE
        | WatchMask::MODIFY
        | WatchMask::DELETE
        | WatchMask::ACCESS
        | WatchMask::CLOSE_WRITE
        | WatchMask::MOVE_SELF;
    inotify
        .watches()
        .add(last_arg, watch_mask)
        .expect("Failed to add watches");


    let mut buffer = [0; 1024];
    loop {
        println!("waiting for input...");
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read event");
        for event in events {
            match event.mask {
                mask if mask.contains(EventMask::CREATE) => {
                    println!("File created");
                }
                mask if mask.contains(EventMask::ACCESS) => {
                    println!("File accessed");
                }
                mask if mask.contains(EventMask::MODIFY) => {
                    println!("File modified");
                }
                mask if mask.contains(EventMask::DELETE) => {
                    println!("File deleted");
                }
                mask if mask.contains(EventMask::MOVE_SELF) => {
                    println!("File moved");
                }
                _ => ()
            }
        }
    }
}

extern crate inotify;


use std::env;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};


fn main() {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    let file_to_monitor = "/proc/1/root/tmp/inotify_file"; // This indicate that it will resolve the real file which is pointing to the real file.

    inotify
        .watches()
        .add(
            file_to_monitor,
            WatchMask::ALL_EVENTS, // We can modify this only care about the 
        )
        .expect("Failed to add inotify watch");

    let file_to_monitor = "/proc/2/root/tmp/inotify_file"; // This indicate that it will resolve the real file which is pointing to the real file.

    inotify
        .watches()
        .add(
            file_to_monitor,
            WatchMask::ALL_EVENTS,
        )
        .expect("Failed to add inotify watch");

    let file_to_monitor = "/tmp/inotify_file"; // This indicate that it will resolve the real file which is pointing to the real file.

    inotify
        .watches()
        .add(
            file_to_monitor,
            WatchMask::ALL_EVENTS,
        )
        .expect("Failed to add inotify watch");

        
    let file_to_monitor = "/tmp/inotify_file2"; // This indicate that it will resolve the real file which is pointing to the real file.

    inotify
        .watches()
        .add(
            file_to_monitor,
            WatchMask::ALL_EVENTS,
        )
        .expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            println!("File inotify: {:?}", event);
        }
    }
}

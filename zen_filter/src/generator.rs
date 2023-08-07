// Generate the Event
use tokio::select;
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;
use tokio::sync::mpsc;
use crate::event::{self, Event};
pub struct Generator {
    pub tx: mpsc::Sender<Box<Event>>
}

impl Generator {
    pub fn new(tx: mpsc::Sender<Box<Event>>) -> Generator {
        Generator {
            tx,
        }
    }

    pub async fn run(mut self) {
        let Self {
            tx
        } = self;

        println!("Start the generator");

        let mut idx = 0;
        loop {
            sleep(Duration::from_millis(rand::thread_rng().gen_range(0..1_000)));
            let event = Generator::generate_event(idx);
            idx = idx + 1;
            if let Err(e) = tx.send(Box::new(event)).await {
                println!("Failed to send event to next component. Error: {e:?}");
            }
        }
    }

    fn generate_event(idx: u32) -> Event {
        let pid = rand::thread_rng().gen_range(1..10);
        let uid = rand::thread_rng().gen_range(10..20);
        let exe = String::from("AAA");
        let r = Event {
            pid,
            uid,
            exe,
            idx,
        };

        println!("Generate event {r:?}");
        r
    }
}
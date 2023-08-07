use tokio::select;
use tokio::sync::mpsc;
use crate::event::Event;
pub struct Filter {
    pub rx: mpsc::Receiver<Box<Event>>,
}

impl Filter {
    pub fn new(rx: mpsc::Receiver<Box<Event>>) -> Filter { 
        Filter {
            rx,
        }
    }

    pub async fn run(mut self) {
        let Self {
            mut rx
        } = self;

        println!("Filter starts to run");

        let mut idx = 0;
        loop {
            select! {
                m = rx.recv() => {
                    if let Some(b) = m {
                        println!("Receive event {b:?}");
                    } else {
                        println!("Receive None ");
                    }
                }
            }
        }
    }
}
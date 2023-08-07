use tokio::sync::mpsc;
use zen_filter::{generator::Generator, filter::Filter};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);
    let generator = Generator::new(tx);
    let filter = Filter::new(rx);
    
    tokio::spawn( async move {filter.run().await});
    tokio::spawn( async move {generator.run().await});
}

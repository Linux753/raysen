use ray_tracing_we;
use std::time::{Instant, Duration};

fn main() {
    let now = Instant::now();
    ray_tracing_we::run();
    eprintln!("This render took : {} ms", now.elapsed().as_millis());
}






use std::time::{self, Duration};
use std::time::Instant;

pub struct Timer {
    title: &'static str,
    time: Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("{} ended in {}ms", self.title, self.time.elapsed().as_millis());
    }
}

impl Timer {
    pub fn new(title: &'static str) -> Timer {
        let start = Instant::now();
        Timer {
            title,
            time: start,
        }
    }
}
use std::time::Instant;

pub struct Timer {
    title: &'static str,
    time: Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        let micros = self.time.elapsed().as_micros();
        let millis = self.time.elapsed().as_millis();

        if micros < 10000 {
            println!("{} ended in {}us", self.title, micros);
        } else {
            println!("{} ended in {}ms", self.title, millis);
        }
    }
}

impl Timer {
    pub fn new(title: &'static str) -> Timer {
        let start = Instant::now();
        Timer { title, time: start }
    }
}

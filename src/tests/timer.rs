use std::future::Future;


pub enum TimerUnit {
    Micros,
    Millis,
}

impl TimerUnit {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TimerUnit::Micros => "μs",
            TimerUnit::Millis => "ms",
        }
    }
}

pub struct Timer {
    start_time: std::time::Instant,
}



impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn elapsed(&self) -> (u128, TimerUnit) {
        let elapsed_micros = self.start_time.elapsed().as_micros();
        if elapsed_micros < 1000 {
            return (elapsed_micros, TimerUnit::Micros);
        }
        let elapsed_millis = self.start_time.elapsed().as_millis();
        return (elapsed_millis, TimerUnit::Millis);
    }
    pub fn print_elasped(&self, message: &str) {
        let (time, unit) = self.elapsed();
        println!("{}: {}{}", message, time, unit.as_str());
    }
    pub fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
    }
}
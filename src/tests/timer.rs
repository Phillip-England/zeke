
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::tests::test::TestLogs;



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
    log_root_dir: String,
}



impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            log_root_dir: "logs".to_string(),
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
    pub fn print_elasped(&mut self, message: &str) {
        let (time, unit) = self.elapsed();
        println!("{}: {}{}", message, time, unit.as_str());
        self.log(TestLogs::HttpTest, message);
        // self.reset();
    }
    pub fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
    }
    pub fn clean_log(&self, file_name: TestLogs) {
        let file_path = format!("{}/{}", self.log_root_dir, file_name.as_str());
        let directory = Path::new(&self.log_root_dir);
    
        if !directory.exists() {
            fs::create_dir_all(directory).expect("Unable to create directory");
        }
    
        let mut _file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // This option truncates the file to zero length if it exists.
            .open(&file_path)
            .expect("Unable to open file");
    }
    pub fn log(&self, file_name: TestLogs, message: &str) {
        let file_path = format!("{}/{}", self.log_root_dir, file_name.as_str());
        let directory = Path::new(&self.log_root_dir);
    
        if !directory.exists() {
            fs::create_dir_all(directory).expect("Unable to create directory");
        }
    
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&file_path)
            .expect("Unable to open file");
    
        let (time, unit) = self.elapsed();
        writeln!(file, "{}: {}{}", message, time, unit.as_str()).expect("Unable to write to file");
    }
}
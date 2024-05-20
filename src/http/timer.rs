
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::tests::test::TestLogs;

pub type Times = Vec<Time>;

pub fn print_times(times: &Times) {
    for time in times {
        println!("{}{}", time.time, time.unit.as_str());
    }
}

pub fn log_times(times: &Times, file_name: TestLogs) {
    let timer = Timer::new();
    for index in 0..times.len() {
        let time = &times[index];
        timer.log(file_name, &format!("{}. {}{}", index, time.time, time.unit.as_str()));
    }
}

pub async fn get_time_range(request_times: &Vec<Time>) -> (Time, Time) {
    let mut min = (0, 100000);
    let mut max = (0, 0);
    for index in 0..request_times.len() {
        let time = &request_times[index];
        let micros = time.get_as_micros();
        if micros < min.1 {
            min = (index, micros);
        }
        if micros > max.1 {
            max = (index, micros);
        }
    }
    let min_time = request_times[min.0].clone();
    let max_time = request_times[max.0].clone();
    return (min_time, max_time);
}


#[derive(Debug, Clone)]
pub struct Time {
    pub time: u128,
    pub unit: TimerUnit,
}

impl Time {
    pub fn new(time: u128, unit: TimerUnit) -> Self {
        Self {
            time,
            unit,
        }
    }
    pub fn get_as_micros(&self) -> u128 {
        match self.unit {
            TimerUnit::Micros => self.time,
            TimerUnit::Millis => self.time * 1000,
        }
    }
    pub fn log(&self, file_name: TestLogs, message: &str) {
        let timer = Timer::new();
        timer.log(file_name, &format!("{}: {}{}", message, self.time, self.unit.as_str()));
    }
}

impl Copy for Time {

}

#[derive(Debug, Clone)]
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

impl Copy for TimerUnit {

}

#[derive(Debug, Clone)]
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
    pub fn elapsed_message(&self) -> String {
        let elapsed_micros = self.start_time.elapsed().as_micros();
        if elapsed_micros < 1000 {
            return format!("{}{}", elapsed_micros, "μs");
        }
        let elapsed_millis = self.start_time.elapsed().as_millis();
        return format!("{}{}", elapsed_millis, "ms");
    }
    pub fn elapsed(&self) -> Time {
        let elapsed_micros = self.start_time.elapsed().as_micros();
        if elapsed_micros < 1000 {
            return Time::new(elapsed_micros, TimerUnit::Micros);
        }
        let elapsed_millis = self.start_time.elapsed().as_millis();
        return Time::new(elapsed_millis, TimerUnit::Millis);
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
    
        writeln!(file, "{}", message).expect("Unable to write to file");
    }
}
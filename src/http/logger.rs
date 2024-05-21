use std::{fs::{self, OpenOptions}, path::Path};
use std::io::Write;

use crate::Response;

use super::request::Request;



#[derive(Debug, Clone,)]
pub enum Logs {
	Trace,
	ServerError,
    HttpTest,
	Debug,
}

impl Logs {
    pub fn as_str(&self) -> &'static str {
        match *self {
			Logs::Trace => "trace.log",
			Logs::ServerError => "error.log",
            Logs::HttpTest => "test.log",
			Logs::Debug => "debug.log",
        }
    }
}

impl Copy for Logs {}

#[derive(Debug, Clone)]
pub struct Logger {
	log_root_dir: String,
	spawn_time: std::time::Instant,
	last_logged: std::time::Instant,
}
impl Logger {
	pub fn new() -> Self {
		Self {
			log_root_dir: "logs".to_string(),
			spawn_time: std::time::Instant::now(),
			last_logged: std::time::Instant::now(),
		}
	}
	pub fn reset_log(&self, file_name: Logs) {
		let file_path = format!("{}/{}", self.log_root_dir, file_name.as_str());
		let directory = Path::new(&self.log_root_dir);
		if !directory.exists() {
			fs::create_dir_all(directory).expect("Unable to create directory");
		}
		let mut file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&file_path)
			.expect("Unable to open file");
		writeln!(file, "").expect("Unable to write to file");
	}
    pub fn elapsed(&self) -> String {
        let elapsed = self.spawn_time.elapsed();
		let micros = elapsed.as_micros();
		if micros < 1000 {
			return format!("{}µs", micros);
		}
		let millis = elapsed.as_millis();
		if millis < 1000 {
			return format!("{}ms", millis);
		}
		let seconds = elapsed.as_secs();
		if seconds < 60 {
			return format!("{}s", seconds);
		}
		return format!("{}ms", millis);

    }
	pub fn logged(&self) -> std::time::Duration {
		self.last_logged.elapsed()
	}
    pub fn log(&self, file_name: Logs, message: &str) {
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
    
        writeln!(file, "{:?}: {}", self.elapsed(), message).expect("Unable to write to file");
    }
    pub fn http(&self, file_name: Logs, label: &str, req: &Request, res: &Response) {
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
    
        writeln!(file, "{:?}: {}", self.elapsed(), label).expect("Unable to write to file");
		writeln!(file, "\treq: {:?}", req).expect("Unable to write to file");
		writeln!(file, "\tres: {:?}", res).expect("Unable to write to file");
        writeln!(file, "\n").expect("Unable to write to file");
    }
}

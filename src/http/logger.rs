use std::{fs::{self, OpenOptions}, path::Path};
use std::io::Write;



#[derive(Debug, Clone)]
pub enum Logs {
	Trace,
	ServerError,
    HttpTest,
}

impl Logs {
    pub fn as_str(&self) -> &'static str {
        match *self {
			Logs::Trace => "trace.log",
			Logs::ServerError => "server_error.log",
            Logs::HttpTest => "http_test.log",
        }
    }
}

impl Copy for Logs {}
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
	pub fn new_line(&mut self, file_name: Logs) {
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
	
		writeln!(file, "").expect("Unable to write to file");
	}
}

use rand::Rng;

pub struct Fuzzer {
	pub host: String,
	pub failed: bool,
	pub paths: Vec<String>,
}

impl Fuzzer {
	pub fn new(host: String) -> Fuzzer {
		Fuzzer {
			host: host,
			failed: false,
			paths: vec![],
		}
	}
	pub fn set_paths(&mut self, paths: Vec<String>) -> &mut Fuzzer{
		self.paths = paths;
		return self;
	}
    pub fn rand_req_str(&mut self) -> String {
		let path = self.get_rand_path();
		let headers = self.get_rand_headers();
		let body = "";
		let status_line = path + " HTTP/1.1";
		let request_str = format!("{}\r\n{}\r\n{}", status_line, headers, body);
		return request_str;
    }
	pub fn get_rand_path(&mut self) -> String {
		let mut rng = rand::thread_rng();
		let random_index = rng.gen_range(0..self.paths.len()); 
		let deviate_chance = rng.gen_range(0..100);
		if deviate_chance == 0 {
			return "TODO: how to make this string useful?".to_string();
		}
		return self.paths[random_index].clone();
	}
	pub fn get_rand_headers(&mut self) -> String {
		let mut headers = String::new();
		let mut rng = rand::thread_rng();
		let random_index = rng.gen_range(0..5);
		for _ in 0..random_index {
			let header = self.get_rand_header();
			headers.push_str(&header);
		}
		return headers;
	}
	pub fn get_rand_header(&mut self) -> String {
		let mut rng = rand::thread_rng();
		let random_index = rng.gen_range(0..5);
		let deviate_chance = rng.gen_range(0..100);
		if deviate_chance == 0 {
			return "TODO: how to make this string useful?".to_string();
		}
		let header = match random_index {
			0 => "Host: localhost\r\n",
			1 => "Connection: close\r\n",
			2 => "User-Agent: Mozilla/5.0\r\n",
			3 => "Accept: text/html\r\n",
			4 => "Accept-Language: en-US\r\n",
			_ => "Accept-Encoding: gzip, deflate\r\n",
		};
		return header.to_string();
	}
}
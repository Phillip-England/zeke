

pub struct Cookie {
	pub name: String,
	pub value: String,
	pub expires: Option<String>,
	pub max_age: Option<String>,
	pub domain: Option<String>,
	pub path: Option<String>,
	pub secure: bool,
	pub http_only: bool,
	pub same_site: Option<String>,
}


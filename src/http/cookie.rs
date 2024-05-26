use time::{OffsetDateTime, format_description::well_known::Rfc2822};

pub struct Cookie {
    pub name: String,
    pub value: String,
    pub expires: Option<OffsetDateTime>,
    pub max_age: Option<u64>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Cookie {
        Cookie {
            name: name.to_string(),
            value: value.to_string(),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    pub fn expires(mut self, expires: OffsetDateTime) -> Cookie {
        self.expires = Some(expires);
        self
    }

    pub fn max_age(mut self, max_age: u64) -> Cookie {
        self.max_age = Some(max_age);
        self
    }

    pub fn domain(mut self, domain: &str) -> Cookie {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn path(mut self, path: &str) -> Cookie {
        self.path = Some(path.to_string());
        self
    }

    pub fn secure(mut self, secure: bool) -> Cookie {
        self.secure = secure;
        self
    }

    pub fn http_only(mut self, http_only: bool) -> Cookie {
        self.http_only = http_only;
        self
    }

    pub fn same_site(mut self, same_site: &str) -> Cookie {
        self.same_site = Some(same_site.to_string());
        self
    }

    pub fn to_string(&self) -> String {
        let mut cookie = format!("{}={}", self.name, self.value);

        if let Some(expires) = &self.expires {
            cookie.push_str(&format!("; Expires={}", expires.format(&Rfc2822).unwrap()));
        }

        if let Some(max_age) = &self.max_age {
            cookie.push_str(&format!("; Max-Age={}", max_age));
        }

        if let Some(domain) = &self.domain {
            cookie.push_str(&format!("; Domain={}", domain));
        }

        if let Some(path) = &self.path {
            cookie.push_str(&format!("; Path={}", path));
        }

        if self.secure {
            cookie.push_str("; Secure");
        }

        if self.http_only {
            cookie.push_str("; HttpOnly");
        }

        if let Some(same_site) = &self.same_site {
            cookie.push_str(&format!("; SameSite={}", same_site));
        }

        cookie
    }
}
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url: url,
            host: String::new(),
            port: String::new(),
            path: String::new(),
            searchpart: String::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn searchpart(&self) -> &str {
        &self.searchpart
    }

    fn is_http(&self) -> bool {
        self.url.starts_with("http://")
    }

    fn extract_host(&self) -> String {
        // 1. `http://` を先頭から取り除く
        // 2. ホスト名とパス(とクエリパラメータ)を分割する
        // 3. イテレーターをVec<&str>に変換する ["host"] or ["host", "path with query"]
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, '/').collect();

        match url_parts[0].find(':') {
            Some(index) => url_parts[0][..index].to_string(),
            None => url_parts[0].to_string(),
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, '/').collect();
        match url_parts[0].find(':') {
            Some(index) => url_parts[0][index + 1..].to_string(),
            None => String::new(),
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, '/').collect();

        if url_parts.len() < 2 {
            return String::new();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, '/').collect();

        if url_parts.len() < 2 {
            return String::new();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

        if path_and_searchpart.len() < 2 {
            return String::new();
        }

        path_and_searchpart[1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = String::from("http://example.com");
        let expected = Ok(Url {
            url: url.clone(),
            host: String::from("example.com"),
            port: String::new(),
            path: String::new(),
            searchpart: String::new(),
        });

        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_port() {
        let url = String::from("http://example.com:8080");
        let expected = Ok(Url {
            url: url.clone(),
            host: String::from("example.com"),
            port: String::from("8080"),
            path: String::new(),
            searchpart: String::new(),
        });

        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_path() {
        let url = String::from("http://example.com/path/to/resource");
        let expected = Ok(Url {
            url: url.clone(),
            host: String::from("example.com"),
            port: String::new(),
            path: String::from("path/to/resource"),
            searchpart: String::new(),
        });

        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_port_path() {
        let url = String::from("http://example.com:8080/path/to/resource");
        let expected = Ok(Url {
            url: url.clone(),
            host: String::from("example.com"),
            port: String::from("8080"),
            path: String::from("path/to/resource"),
            searchpart: String::new(),
        });

        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_port_path_searchpart() {
        let url = String::from("http://example.com:8080/path/to/resource?query=param");
        let expected = Ok(Url {
            url: url.clone(),
            host: String::from("example.com"),
            port: String::from("8080"),
            path: String::from("path/to/resource"),
            searchpart: String::from("query=param"),
        });

        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_no_scheme() {
        let url = String::from("example.com");
        let expected = Err(String::from("Only HTTP scheme is supported."));
        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = String::from("https://example.com");
        let expected = Err(String::from("Only HTTP scheme is supported."));
        assert_eq!(expected, Url::new(url).parse())
    }
}

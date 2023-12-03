use std::collections::HashMap;

// HTTPRequest struct
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            // request line
            if line.contains("HTTP") {
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            // header line
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // don't do anything, as it indicates end of headers
            } else {
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body.to_string(),
        }
    }
}

fn process_request_line(s: &str) -> (Method, Resource, Version) {
    let mut request_line = s.split_whitespace();
    let method = request_line.next().unwrap();
    let resource = request_line.next().unwrap();
    let version = request_line.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header.next() {
        value = v.trim().to_string();
    }
    (key, value)
}

// Resource enum
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

// Method enum
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

// Version enum
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let raw_request = String::from(
            "GET / HTTP/1.1\r\n
            Host: example.com\r\n
            Connection: keep-alive\r\n
            \r\n",
        );
        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "example.com".into());
        headers_expected.insert("Connection".into(), "keep-alive".into());
        let req: HttpRequest = raw_request.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}

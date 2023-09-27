use crate::socket::Socket;
use std::collections::HashMap;
use std::fmt;

type StringMap = HashMap<String, String>;
const BUFFER_SIZE: usize = 8;

#[derive(Debug)]
enum RequestMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
    Uninitialized,
}

#[derive(Debug)]
enum HttpError {
    InvalidFormat,
    InvalidMethod,
    InvalidHeader,
    ReceivingFailed,
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::InvalidFormat => write!(f, "Error: invalid HTTP format"),
            HttpError::InvalidMethod => write!(f, "Error: invalid HTTP method"),
            HttpError::InvalidHeader => write!(f, "Error: invalid HTTP header"),
            HttpError::ReceivingFailed => {
                write!(f, "Error: failed to receive HTTP request message")
            }
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    url: String,
    version: String,
    headers: StringMap,
    body: Option<String>,
}

pub struct Response {
    version: String,
    status_code: String,
    status_text: String,
    headers: StringMap,
    body: Option<String>,
}

impl Request {
    fn parse_request_line(line: &str) -> Result<(RequestMethod, String, String), HttpError> {
        let mut parts = line.split_whitespace();
        let method_str = parts.next().ok_or(HttpError::InvalidFormat)?;
        let url = parts.next().ok_or(HttpError::InvalidFormat)?;
        let version = parts.next().ok_or(HttpError::InvalidFormat)?;
        Ok((
            match method_str {
                "CONNECT" => RequestMethod::Connect,
                "DELETE" => RequestMethod::Delete,
                "GET" => RequestMethod::Get,
                "HEAD" => RequestMethod::Head,
                "OPTIONS" => RequestMethod::Options,
                "PATCH" => RequestMethod::Patch,
                "POST" => RequestMethod::Post,
                "PUT" => RequestMethod::Put,
                "TRACE" => RequestMethod::Trace,
                _ => return Err(HttpError::InvalidMethod),
            },
            url.to_string(),
            version.to_string(),
        ))
    }

    fn parse_headers<'a, I>(lines: &mut I) -> Result<StringMap, HttpError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut headers: StringMap = HashMap::new();
        while let Some(line) = lines.next() {
            // \r\n\r\n means that the headers part ends
            if line.is_empty() {
                break;
            }

            let kv: Vec<&str> = line.splitn(2, ':').collect();
            if kv.len() != 2 {
                return Err(HttpError::InvalidHeader);
            }

            headers.insert(kv[0].trim().to_string(), kv[1].trim().to_string());
        }
        Ok(headers)
    }

    fn from_socket(client_socket: &Socket) -> Result<Request, Box<dyn std::error::Error>> {
        let mut method: RequestMethod = RequestMethod::Uninitialized;
        let mut url: String = String::new();
        let mut version: String = String::new();
        let mut headers: StringMap = HashMap::new();
        let mut body: Option<String> = None;

        let mut buffer = [0u8; BUFFER_SIZE];
        let mut request_post: Vec<u8> = Vec::new();
        let mut content_length: Option<usize> = None;
        let mut header_received = false;

        loop {
            match client_socket.read(&mut buffer) {
                Ok(num_of_bytes) if num_of_bytes > 0 => {
                    request_post.extend_from_slice(&buffer[..num_of_bytes as usize]);
                    if !header_received {
                        if let Some(end_index_of_headers) = request_post
                            .windows(4)
                            .position(|slice| slice == b"\r\n\r\n")
                        {
                            header_received = true;
                            // Parse request line and headers
                            let headers_str = String::from_utf8(
                                request_post[..end_index_of_headers + 4].to_vec(),
                            )?;
                            let mut lines = headers_str.lines();
                            
                            let request_line = lines.next().ok_or(HttpError::InvalidFormat)?;
                            (method, url, version) = Request::parse_request_line(request_line)?;
                            headers = Request::parse_headers(&mut lines)?;

                            if let Some(len) = headers.get("Content-Length") {
                                content_length = Some(len.parse::<usize>()?);
                            } else {
                                break;
                            };
                            request_post = request_post[end_index_of_headers + 4..].to_vec()
                        }
                    }
                    if header_received {
                        if let Some(len) = content_length {
                            if request_post.len() >= len {
                                body = Some(String::from_utf8(request_post)?);
                                break;
                            }
                        }
                    }
                }
                _ => return Err(Box::new(HttpError::ReceivingFailed)),
            }
        }
        Ok(Request {
            method,
            url,
            version,
            headers,
            body,
        })
    }
}

pub fn client_handler(client_socket: Socket) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::from_socket(&client_socket)?;
    Ok(())
}

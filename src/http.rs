use crate::socket::Socket;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use http::Uri;

type StringMap = HashMap<String, String>;
const BUFFER_SIZE: usize = 1024;

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
    uri: Uri,
    version: String,
    headers: StringMap,
    body: Option<String>,
}

pub struct Response {
    version: String,
    status_code: String,
    status_text: String,
    headers: StringMap,
    body: Option<Vec<u8>>,
}

impl Request {
    fn parse_request_line(line: &str) -> Result<(RequestMethod, String, String), HttpError> {
        let mut parts = line.split_whitespace();
        let method_str = parts.next().ok_or(HttpError::InvalidFormat)?;
        let uri = parts.next().ok_or(HttpError::InvalidFormat)?;
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
            uri.to_string(),
            version.to_string(),
        ))
    }

    fn from_socket(client_socket: &Socket) -> Result<Request, Box<dyn std::error::Error>> {
        let mut method: RequestMethod = RequestMethod::Uninitialized;
        let mut uri: String = String::new();
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
                            println!("{}", &headers_str);
                            let mut lines = headers_str.lines();

                            let request_line = lines.next().ok_or(HttpError::InvalidFormat)?;
                            (method, uri, version) = Request::parse_request_line(request_line)?;

                            while let Some(line) = lines.next() {
                                // \r\n\r\n means that the headers part ends
                                if line.is_empty() {
                                    break;
                                }

                                let kv: Vec<&str> = line.splitn(2, ':').collect();
                                if kv.len() != 2 {
                                    return Err(Box::new(HttpError::InvalidHeader));
                                }

                                headers.insert(kv[0].trim().to_string(), kv[1].trim().to_string());
                            }

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
                                println!("{}", body.as_deref().unwrap());
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
            uri: uri.parse::<http::Uri>()?,
            version,
            headers,
            body,
        })
    }

    fn response(&self, root_path: &PathBuf) -> Result<Response, Box<dyn std::error::Error>> {
        let mut headers: StringMap = HashMap::new();
        let version = "HTTP/1.1".to_string();
        // let current_time = SystemTime::now();
        // let unix_timestamp = current_time.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        match self.method {
            RequestMethod::Connect => todo!(),
            RequestMethod::Delete => todo!(),
            // GET
            // Request has body:    No
            // Successful response has body: Yes
            RequestMethod::Get => {
                let path = root_path.join(std::path::Path::strip_prefix(&PathBuf::from(self.uri.path()), "/")?);
                Ok(match std::fs::read(&path) {
                    Ok(bytes) => {
                        match path.extension().and_then(OsStr::to_str) {
                            Some(ext) => {
                                headers.insert(
                                "Content-Type".to_string(),
                                match ext {
                                    "aac" => "audio/aac",
                                    "avi" => "video/x-msvideo",
                                    "bin" => "application/octet-stream",
                                    "bmp" => "image/bmp",
                                    "bz" => "application/x-bzip",
                                    "bz2" => "application/x-bzip2",
                                    "css" => "text/css",
                                    "csv" => "text/csv",
                                    "doc" => "application/msword",
                                    "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                                    "epub" => "application/epub+zip",
                                    "gif" => "image/gif",
                                    "htm" | "html" => "text/html",
                                    "ico" => "image/vnd.microsoft.icon",
                                    "jpg" | "jpeg" => "image/jpeg",
                                    "js" => "text/javascript",
                                    "json" => "application/json",
                                    "mp3" => "audio/mpeg",
                                    "png" => "image/png",
                                    "pdf" => "application/pdf",
                                    "ppt" => "application/vnd.ms-powerpoint",
                                    "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                                    "rar" => "application/x-rar-compressed",
                                    "txt" => "text/plain",
                                    "wav" => "audio/wav",
                                    "webp" => "image/webp",
                                    "xml" => "application/xml",
                                    "zip" => "application/zip",
                                    _ => "application/octet-stream"
                                }.to_string(),
                            );
                            }
                            None => (),
                        }
                        Response {
                            version,
                            status_code: "200".to_string(),
                            status_text: "OK".to_string(),
                            headers,
                            body: Some(bytes),
                        }
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => Response {
                            version,
                            status_code: "404".to_string(),
                            status_text: "Not Found".to_string(),
                            headers,
                            body: None,
                        },
                        std::io::ErrorKind::PermissionDenied => Response {
                            version,
                            status_code: "403".to_string(),
                            status_text: "Forbidden".to_string(),
                            headers,
                            body: None,
                        },
                        _ => Response {
                            version,
                            status_code: "500".to_string(),
                            status_text: "Internal Server Error".to_string(),
                            headers,
                            body: None,
                        },
                    },
                })
            }
            RequestMethod::Head => todo!(),
            RequestMethod::Options => todo!(),
            RequestMethod::Patch => todo!(),
            RequestMethod::Post => todo!(),
            RequestMethod::Put => todo!(),
            RequestMethod::Trace => todo!(),
            RequestMethod::Uninitialized => todo!(),
        }
    }
}

impl Response {
    fn send(&self, client_socket: &Socket) -> Result<(), Box<dyn std::error::Error>> {
        let mut response_line_str =
            format!("{} {} {}", self.version, self.status_code, self.status_text);
        for (k, v) in &self.headers {
            response_line_str = format!("{}\r\n{}: {}", response_line_str, k, v);
        }
        response_line_str = response_line_str + "\r\n\r\n";
        client_socket.write(response_line_str.as_bytes())?;

        match &self.body {
            Some(bytes) => {
                if bytes.len() <= BUFFER_SIZE {
                    client_socket.write(bytes)?;
                } else {
                    let len = bytes.len();
                    let mut seperate: usize = 0;
                    while seperate <= len {
                        if seperate + BUFFER_SIZE <= len {
                            client_socket.write(&bytes[seperate..seperate + BUFFER_SIZE])?;
                        } else {
                            client_socket.write(&bytes[seperate..seperate])?;
                        }
                        seperate = seperate + BUFFER_SIZE;
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }
}

pub fn client_handler(
    client_socket: Socket,
    root_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::from_socket(&client_socket)?;
    let response = request.response(root_path)?;
    response.send(&client_socket)
}

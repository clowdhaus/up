use std::{
  io::{Read, Write},
  net::{Shutdown, SocketAddr, TcpStream},
};

fn main() -> std::io::Result<()> {
  // Connect to server
  let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
  let mut stream = TcpStream::connect(addr)?;

  // Create GET request
  let path = "/healthz";
  let mut http_req = String::new();
  http_req.push_str(&format!("GET {} HTTP/1.1\r\n", path));
  http_req.push_str(&format!("Host: {}\r\n", addr));
  http_req.push_str("Connection: close\r\n");
  http_req.push_str("\r\n");

  // Send request
  let req_bytes = http_req.as_bytes();
  stream.write_all(&req_bytes)?;

  // Parse response
  let mut response = String::new();
  stream.read_to_string(&mut response)?;

  stream.shutdown(Shutdown::Write)?;
  Ok(())
}

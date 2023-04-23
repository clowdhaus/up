use std::{
  env,
  io::{Read, Write},
  net::{Shutdown, SocketAddr, TcpStream},
};

fn get_path(args: &[String]) -> String {
  match env::var("UP_PATH") {
    Ok(path) => path,
    Err(_) => {
      if args.len() > 2 {
        args[1].to_string()
      } else {
        "/healthz".to_string()
      }
    }
  }
}

fn get_port(args: &[String]) -> u16 {
 match env::var("UP_PORT") {
    Ok(port) => port.parse::<u16>().unwrap(),
    Err(_) => {
      if args.len() > 2 {
        args[2].parse::<u16>().unwrap()
      } else {
        8080
      }
    }
  }
}

fn main() -> std::io::Result<()> {
  let args = env::args().collect::<Vec<String>>();

  let port = get_port(&args);
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  let mut stream = TcpStream::connect(addr)?;

  // Create GET request
  let path = get_path(&args);
  let mut http_req = String::new();
  http_req.push_str(&format!("GET {} HTTP/1.1\r\n", path));
  http_req.push_str(&format!("Host: {}\r\n", addr));
  http_req.push_str("Connection: close\r\n");
  http_req.push_str("\r\n");

  // Send request
  let req_bytes = http_req.as_bytes();
  stream.write_all(req_bytes)?;

  // Parse response
  let mut response = String::new();
  stream.read_to_string(&mut response)?;

  stream.shutdown(Shutdown::Write)?;
  Ok(())
}

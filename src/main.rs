use std::{
  env,
  io::{Read, Write},
  net::{Shutdown, SocketAddr, TcpStream},
};

/// Port and path arguments that can be passed as
/// --port <...> and --path <...>
struct Args {
  port: Option<u16>,
  path: Option<String>,
}

/// Parse any arguments provided to the executable
fn parse_args() -> Result<Args, pico_args::Error> {
  let mut pargs = pico_args::Arguments::from_env();

  let args = Args {
    port: pargs.opt_value_from_str("--port")?,
    path: pargs.opt_value_from_str("--path")?,
  };
  let _ = pargs.finish();

  Ok(args)
}

/// Get the path from either the environment variable or parsed arguments
fn get_path(args: &Args) -> String {
  match env::var("UP_PATH") {
    Ok(path) => path,
    Err(_) => match &args.path {
      Some(path) => path.to_owned(),
      None => "/healthz".to_string(),
    },
  }
}

/// Get the port from either the environment variable or parsed arguments
fn get_port(args: &Args) -> u16 {
  match env::var("UP_PORT") {
    Ok(port) => port.parse::<u16>().unwrap(),
    Err(_) => args.port.unwrap_or(80),
  }
}

fn main() -> std::io::Result<()> {
  let args = match parse_args() {
    Ok(v) => v,
    Err(e) => {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
  };

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

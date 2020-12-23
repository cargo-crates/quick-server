use std::net::{SocketAddr, TcpListener};
// use std::net::TcpStream;
use std::io;

pub struct Server {
  addr: SocketAddr,
  listener: Option<TcpListener>
}

impl Server {
  pub fn new(addr: SocketAddr) -> Server {
    Server { addr, listener: None }
  }
  pub fn start(&mut self) -> Result<&Server, io::Error> {
    let listener = TcpListener::bind(&self.addr)?;
    for stream in listener.incoming() {
      let _stream = stream.unwrap();
      println!("Connection established!");
    }
    self.listener = Some(listener);
    Ok(self)
  }
}




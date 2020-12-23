### Quick Start

```rust
use std::net::{ SocketAddr };
use quick_server::server::Server;

fn main() {
    // We'll bind to 127.0.0.1:3900
    let addr = SocketAddr::from(([127, 0, 0, 1], 3900));
    let _server = match Server::new(addr).start() {
        Ok(server) => server,
        Err(e) => panic!("err: {}", e),
    };
}
```
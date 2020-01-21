use std::io;

fn main() -> io::Result<()> {
    eviov_server::start::<Plugin>()
}

struct Plugin;

impl eviov_server::Plugin for Plugin {}

pub struct Handler;

impl super::Handler for Handler {
    fn on_error(&self, error: String) {
        unimplemented!()
    }

    fn on_close(&self, error: &str) {
        unimplemented!()
    }

    fn on_message(&self, bytes: Vec<u8>) {
        unimplemented!()
    }
}

pub enum QueryError {
    SocketClosed,
}

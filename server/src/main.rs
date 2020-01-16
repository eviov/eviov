use std::io;

dirmod::all!(default pub(self));

#[actix_rt::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    ws::start().await
}

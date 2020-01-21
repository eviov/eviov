use std::io;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use actix::prelude::*;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod config;

pub async fn start() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(entry)
    })
    .bind("0.0.0.0:15678")?
    .run()
    .await
}

struct SessionIdCount(AtomicU32);

impl SessionIdCount {
    pub fn next(&self) -> u32 {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

#[actix_web::get("/")]
async fn entry(
    r: HttpRequest,
    stream: web::Payload,
    sic: web::Data<SessionIdCount>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(Session::new(sic.next()), &r, stream)
}

struct Session {
    session_id: u32,
    last_ping: Instant,
}

impl Session {
    pub fn new(session_id: u32) -> Self {
        Self {
            session_id,
            last_ping: Instant::now(),
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(config::RUN_INTERVAL, move |act, ctx| {
            if act.last_ping.elapsed() > config::TIMEOUT {
                log::info!("Session {}: client timeout", act.session_id);
                ctx.stop();
                return;
            }

            ctx.ping(&[]);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_ping = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_ping = Instant::now();
            }
            Ok(ws::Message::Text(_)) => {
                log::warn!(
                    "Received unexpected text message from session {}",
                    self.session_id
                );
            }
            Ok(ws::Message::Binary(data)) => {
                let _data = match rmp_serde::from_read_ref(&data) {
                    Ok(data) => data,
                    Err(_) => {
                        log::info!("Session {}: Received malformed data", self.session_id);
                        ctx.stop();
                        return;
                    }
                };
                // TODO delegate data
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

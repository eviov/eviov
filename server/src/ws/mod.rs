use std::io;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

use actix::prelude::*;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::Plugin;

mod config;

pub async fn start<X: Plugin>(plugin: Arc<X>) -> io::Result<()> {
    let plugin = web::Data::new(plugin);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&plugin))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(entry::<X>)))
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

async fn entry<X: Plugin>(
    r: HttpRequest,
    stream: web::Payload,
    sic: web::Data<SessionIdCount>,
    plugin: web::Data<Arc<X>>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(Session::new(sic.next(), Arc::clone(&**plugin)), &r, stream)
}

struct Session<X: Plugin> {
    session_id: u32,
    last_ping: Instant,
    plugin: Arc<X>,
}

impl<X: Plugin> Session<X> {
    pub fn new(session_id: u32, plugin: Arc<X>) -> Self {
        Self {
            session_id,
            last_ping: Instant::now(),
            plugin,
        }
    }
}

impl<X: Plugin> Actor for Session<X> {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let _ = ctx.run_interval(config::RUN_INTERVAL, move |act, ctx| {
            if act.last_ping.elapsed() > config::TIMEOUT {
                log::info!("Session {}: client timeout", act.session_id);
                ctx.stop();
                return;
            }

            ctx.ping(&[]);
        });
    }
}

impl<X: Plugin> StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session<X> {
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
                // TODO delegate data
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{HttpResponse, web, Result, HttpRequest, Error};
use actix_web_actors::ws;
use chrono::Local;

// calculate online counter
static ONLINE: AtomicI64 = AtomicI64::new(0);


struct WS;

// implement 1
impl Actor for WS {
    type Context = ws::WebsocketContext<Self>;
}

// implement 2
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("receive: {:?} {}", text, Local::now().time());
                let res = ONLINE.load(Ordering::SeqCst);
                ctx.text(format!("{:?}", res))
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
    // hook
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("socket connected!");
        ONLINE.fetch_add(1, Ordering::SeqCst);
        let res = ONLINE.load(Ordering::SeqCst);
        ctx.text(format!("{:?}", res))
    }
    // hook
    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("socket closed!");
        ONLINE.fetch_sub(1, Ordering::SeqCst);
        ctx.stop()
    }
}

pub async fn calculate_online(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WS{}, &req, stream);
    resp
}

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use super::ONLINE;
    #[test]
    fn test_online() {
        let res= ONLINE.fetch_and(1, Ordering::SeqCst);
        println!("count: {:?}", ONLINE)
    }
}
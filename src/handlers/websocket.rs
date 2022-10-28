use actix::{Actor, StreamHandler};
use actix_web::{HttpResponse, web, Result, HttpRequest, Error};
use actix_web_actors::ws;

struct Ws;

// implement 1
impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

// implement 2
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("{:?}", text);
                ctx.text("1111")
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn calculate_online(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(Ws{}, &req, stream);
    resp
}
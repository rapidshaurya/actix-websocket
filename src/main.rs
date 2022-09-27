use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, Responder, HttpServer};
use actix_web_actors::ws::{self};
use actix_files::NamedFile;
/// Define HTTP actor
struct MyWs;

//adding actor trait to MyWs struct
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("{:?}\nAddress{:?}", msg, ctx.address());
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                println!("{}\nAddress{:?}", text, ctx.address());
                ctx.text(text)},
            Ok(ws::Message::Binary(bin)) => {
                println!("{:?}\nAddress{:?}", bin, ctx.address());
                ctx.binary(bin)},
            _ => (),
        }
    }
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("New Client detected ");
        ctx.text("welcome to chatbox");
    }
    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("Client Disconneted");
        ctx.text("disconnected0");
    }
}

//adding html page to http://127.0.0.1:8080/
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

//websocket route
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(web::resource("/").to(index))
            // Add the WebSocket route
    .service(web::resource("/ws").route(web::get().to(websocket)))
         )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
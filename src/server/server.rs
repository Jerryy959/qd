use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use std::net::SocketAddr;
use std::io::Result;

fn is_port_available(port: u16) -> Result<bool> {
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  match std::net::TcpListener::bind(&addr) {
      Ok(_) => Ok(true), 
      Err(_) => Ok(false),
  }
}

#[derive(Deserialize)]
pub struct Info {
  pub name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
  "Hello, world!"
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
  format!("Hello, {}!", info.name)
}

pub async fn start_server(port: u16) -> std::io::Result<()> {
  if !is_port_available(port)? {
    return Err(std::io::Error::new(
      std::io::ErrorKind::AddrInUse,
      "Port is already in use",
    ));
  }

  let addr = format!("127.0.0.1:{}", port);
  println!("Starting server on {}", addr);

  HttpServer::new(|| {
      App::new()
          .service(hello)
          .service(echo)
  })
  .bind(&addr)?
  .run()
  .await
}


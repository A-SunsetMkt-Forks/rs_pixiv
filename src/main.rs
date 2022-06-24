mod tmplate;
mod services;
mod download;
mod json_struct;

use log::{ info, LevelFilter};
use actix_web::{ web, App, HttpServer, Responder};
use std::env;


struct AppState {
    client:awc::Client
}



#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).try_init().unwrap();
    let port: u16 = match env::var("PORT") {
        Ok(p) => {
            p.parse().unwrap()
        }
        Err(_) => {
            8080
        }
    };
    info!("Run server on port {}",port);
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(
            AppState{
               client: awc::Client::default()
            }
        ))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::ErrorHandlers::default())
      
            .service(services::index)
            .service(services::json_img)
            .service(services::web_img)
            .service(services::raw_img)

    })
        .bind(("0.0.0.0", port))?
        .bind(("::",port))?
        .run()
        .await
}


use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
mod api_endpoints;
use api_endpoints::configure_path;
use colored::Colorize;
mod shared_ops;
use shared_ops::SharedCredAndOps;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = aws_apis::load_from_env().await;
    let port_arg = std::env::args().skip(1).nth(1);
    let port = std::env::var("PORT_TO_RUN");
    let port_to_run = match port {
        Ok(port) => port,
        Err(_) => {
            println!("There is no '{}' environment variable to find; I am trying to read from the CLI argument","PORT_TO_RUN".yellow().bold());
            match port_arg {
                Some(port) => port,
                None => {
                    println!("Also, no port value was given as an argument, so the server is now running on: '{}'","127.0.0.1:8090".green().bold());
                    "127.0.0.1:8090".into()
                }
            }
        }
    };
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8090")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .configure(configure_path)
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(SharedCredAndOps::initialize_cred_and_ops(
                config.clone(),
            )))
            .default_service(web::get().to(|| async { "Bad Gateway" }))
    })
    .bind(&port_to_run)?
    .run()
    .await
}

use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web::Html};
use anyhow::Result;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_WEBSITE_URL: &str = "https://en.wikipedia.org/wiki/HTTP_404";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.3";

#[get("/")]
async fn index() -> impl Responder {
    match get_website_html().await {
        Ok(html) => {
            let content = Html::new(html);
            content
        }
        Err(err) => {
            println!("{}", err);
            Html::new("<p>Unable to get page</p>")
        }
    }
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Version", "0.0.1"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .map(|port| match port.parse() {
            Ok(int) => int,
            Err(_e) => {
                println!("Invalid PORT. Using default value.");
                DEFAULT_PORT
            }
        })
        .unwrap_or(DEFAULT_PORT);

    println!("Starting HTTP server at 0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn get_website_html() -> Result<String> {
    let url = env::var("WEBSITE_URL");

    match url {
        Ok(str) => {
            print!("website_url: {}", str);
            let response = minreq::get(str)
                .with_header("User-Agent", USER_AGENT)
                .send()?;
            let body = response.as_str()?;
            Ok(body.to_owned())
        }
        Err(e) => {
            println!(
                "Error while getting Website URL. Using default value. {}",
                e
            );
            let response = minreq::get(DEFAULT_WEBSITE_URL)
                .with_header("User-Agent", USER_AGENT)
                .send()?;
            let body = response.as_str()?;
            Ok(body.to_owned())
        }
    }
}

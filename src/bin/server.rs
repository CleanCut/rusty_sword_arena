use std::time::{Duration, Instant};
use std::thread::spawn;

extern crate actix_web;
use actix_web::{http, server, App, Path, Responder, HttpResponse};
use actix_web::http::{header, Method, StatusCode};

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn default(_info: Path<()>) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("We're up. <a href=\"/stop\">Click here to stop</a>")
}

fn stop(_info: Path<()>) -> impl Responder {
    std::process::exit(0);
    ""
}

fn main() {

    let game_loop = spawn(|| {
        let mut loop_iterations : i64 = 0;
        let mut report_starttime = Instant::now();
        let report_frequency = Duration::new(1, 0);
        loop {
            loop_iterations += 1;

            // Print out stats
            if report_starttime.elapsed() > report_frequency {
                println!("FPS: {}", loop_iterations);
                loop_iterations = 0;
                report_starttime = Instant::now();
            }
        }
    });

    server::new(
        || App::new()
            .route("/{id}/{name}/index.html", http::Method::GET, index)
            .route("/", http::Method::GET, default)
            .route("/stop", http::Method::GET, stop)
    )
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(0)
        .run();

    game_loop.join().unwrap();
}
/*
初始化
*/
use actix_files as fs;
use actix_web::middleware::{Logger, TrailingSlash::Trim};
use actix_web::{dev::Service as _, web, App, HttpServer};
use env_logger::{Builder, Env};
use futures_util::future::FutureExt;
use lazy_static::lazy_static;
use logger_rust::*;
use std::thread;
use std::time::Duration;
use tera::Tera;
#[path = "controller/PageController.rs"]
mod page;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("assets/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

pub fn init_logger() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
}
pub async fn create_server(server_ip: &str) -> std::io::Result<()> {
    init_logger();
    log_info!("🚀 Trying to run on: \x1b[31m{}\x1b[0m", server_ip);
    let server = match HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                //println!("Hi from start. You requested: {}", req.path());
                //req.extensions_mut().insert("val".to_string());
                srv.call(req).map(|res| {
                    //println!("Hi from response");
                    res
                })
            })
            .app_data(web::Data::new(TEMPLATES.clone()))
            .service(
                fs::Files::new("/static", "assets/static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .wrap(Logger::default())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(Trim))
            //首页
            .route("/", web::get().to(page::index))
            //.service(web::resource("/upload").route(web::post().to(save_file))) <-- i'll add this bullshit somehow later maybe
            //.service(web::resource("/data").route(web::get().to(get_data))) <-- i'll add this bullshit somehow later maybe
            // i'll add a db connection here later too but idk why then this server called "basic" lol
            .app_data(web::Data::new(page::error_handler))
            .default_service(actix_web::web::route().to(page::not_found)) // default gateway for bad request -> like 404
    })
    .bind(server_ip)
    {
        // for ok
        Ok(server) => {
            // if ok
            log_warn!(
                "📢 \x1B[1m\x1b[32mListening on: \x1b[31mhttp://{}\x1b[0m",
                server_ip
            ); // print the server IP address after the server starts
            log_info!("✅ \x1B[1m\x1B[4mOk bro now i'm gonna run ur site\x1b[0m");
            server
        }
        // for errors
        Err(e) => {
            log_error!("!!! FAILED TO BIND A SERVER !!!\n\x1b[33mIP: \x1b[31m'{}'\n\x1b[33m  |\n\x1b[33m  v\n\x1b[33mERROR_CODE: \x1b[31m{}\x1b[0m", server_ip, e);
            thread::sleep(Duration::from_secs(10));
            return Err(e);
        }
    };
    server.run().await?;
    Ok(())
}

// colors:
// \x1b[32m - green
// \x1b[31m - red
// \x1B[4m - underline
// \x1B[1m - bold
// \x1b[0m - reset

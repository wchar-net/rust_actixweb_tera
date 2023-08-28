use actix_web::{dev::ServiceRequest, error, web, HttpResponse, Responder};
use tera::{Context, Tera};

/**
 * 首页界面
 */
pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "demo");
    let template = tera.render("pages/home.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

/**
 * error
 */
pub async fn error_handler(
    tera: web::Data<Tera>,
    _req: ServiceRequest,
    _error: error::Error,
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "error");
    let template = tera.render("error/500.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

/**
 * 404
 */
pub async fn not_found(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "404");
    let template = tera.render("error/404.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

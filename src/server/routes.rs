use super::handlers;
use actix_web::web;
use actix_web::web::ServiceConfig;

/// Configures the routes/services for Server
pub fn config(cfg: &mut ServiceConfig) {
    cfg.route(
        "/.well-known/matrix/client",
        web::get().to(handlers::admin::get_wellknown),
    )
    .route(
        "/_matrix/client/version",
        web::get().to(handlers::admin::get_versions),
    )
    .service(web::scope("/_matrix/client/r0"));
}
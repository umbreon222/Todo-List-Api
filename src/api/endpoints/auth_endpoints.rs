use actix_web::web;

// Authentication endpoint configuration callback
pub fn auth_endpoints(config: &mut web::ServiceConfig) {
    /*config
        .service(
            web::resource("/authorize")
                .route(web::get().to(get_authorize))
                .route(web::post().to(post_authorize)),
        )
        .route("/token", web::post().to(token))*/
}
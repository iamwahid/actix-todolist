use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::{MySqlPool};

use crate::routes::health_check;
// use crate::routes::subscribe;
use crate::routes::{activity_create, activity_list, activity_detail, activity_update, activity_destroy};
use crate::routes::{todo_detail, todo_list, todo_create, todo_destroy, todo_update};
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, pool: MySqlPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(activity_list)
            .service(activity_detail)
            .service(activity_create)
            .service(activity_update)
            .service(activity_destroy)
            .service(todo_list)
            .service(todo_create)
            .service(todo_detail)
            .service(todo_update)
            .service(todo_destroy)
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
use actix_web::{web, App, HttpServer};

mod postgres;
mod routes;
mod user;
mod task;

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_down(&pg_pool).await; //開発時のみ使う
    postgres::migrate_up(&pg_pool).await;

    let address = address();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(routes::list_users)
            .service(routes::create_task)
    })
    .bind(&address)?
    .run()
    .await
}

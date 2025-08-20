use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod dto;
mod handlers;
mod models;

use handlers::{
    create_product, get_all_products, get_product_by_id, soft_delete_product, update_product,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_product)
            .service(get_all_products)
            .service(get_product_by_id)
            .service(update_product)
            .service(soft_delete_product)
        // .service(update_product)
        // .service(soft_delete_product)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

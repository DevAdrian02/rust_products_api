mod entity;
mod handlers;
mod models;
mod middleware;
mod utils;

use actix_web::{web, App, HttpServer, middleware as actix_middleware, HttpResponse};
use sea_orm::Database;
use std::env;
use dotenvy::dotenv;
use crate::handlers::product_handler::*;
use crate::middleware::auth::AuthMiddleware;
use log;
use futures;
use serde_json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Configuración de Entorno y Logs
    dotenv().ok();
    // Inicializa el logger para ver logs de Actix y tus propios logs
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Variables de Entorno
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL no configurada en .env");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Conexión a la Base de Datos
    log::info!("Intentando conectar a la base de datos...");
    let db = Database::connect(&db_url).await.expect("Error conectando a la BD. Revise su DATABASE_URL.");

    log::info!("Conexión exitosa. Servidor corriendo en http://{}:{}", host, port);

    // Inicialización del Servidor Actix-Web
    HttpServer::new(move || {
        App::new()
            // Inyectar la conexión a la base de datos
            .app_data(web::Data::new(db.clone()))
            // Middleware de logs
            .wrap(actix_middleware::Logger::default())

            // RUTA PÚBLICA (GENERADOR DE TOKEN)
            // Esta ruta se define primero y está FUERA de cualquier .wrap(AuthMiddleware).
            .route("/api/v1/auth/generate_token", web::post().to(|| async {
                match crate::utils::jwt_utils::create_token("test_evaluator") {
                    Ok(token) => {
                        HttpResponse::Ok().json(serde_json::json!({
                            "token": token,
                            "note": "Use this token in Authorization: Bearer [token]"
                        }))
                    },
                    Err(e) => HttpResponse::InternalServerError().body(e),
                }
            }))

            // CONFIGURACIÓN DEL ALCANCE PROTEGIDO
            // uso web::scope("") para aplicar el middleware SOLO a las rutas de CRUD.
            .service(
                web::scope("")
                    .wrap(AuthMiddleware)
                    // SERVICIOS PROTEGIDOS
                    .service(create_product)
                    .service(get_products)
                    .service(get_product_by_id)
                    .service(update_product)
                    .service(delete_product)
            )

    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
//! Punto de entrada de la aplicación Actix Web.
//! Configura y ensambla dependencias, inicia el servidor HTTP.

use actix_web::{web, App, HttpServer, middleware};
use log::info;
use std::env;

use domain::config::Config;
use domain::application::services::TaskService;
use domain::infrastructure::repositories::TaskRepository;
use domain::infrastructure::handlers::task_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    info!("Iniciando aplicación de gestión de tareas...");

    // Cargar configuración desde variables de entorno
    let config = Config::from_env()
        .expect("Error al cargar configuración desde variables de entorno");

    info!("Configuración cargada correctamente");

    // Crear pool de conexión a la base de datos
    let pool = config.create_pool()
        .await
        .expect("Error al crear el pool de conexiones a la base de datos");

    info!("Pool de conexiones a la base de datos creado");

    // Crear repositorio y servicio
    let task_repository = TaskRepository::new(pool.clone());
    let task_service = TaskService::new(task_repository);

    info!("Servicios de dominio inicializados");

    // Iniciar servidor HTTP
    let bind_address = format!("{}:{}", 
        env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    );

    info!("Iniciando servidor en {}", bind_address);

    HttpServer::new(move || {
        App::new()
            // Middleware de logging
            .wrap(middleware::Logger::default())
            // Middleware de compresión
            .wrap(middleware::Compress::default())
            // Datos compartidos (repositorio y servicio)
            .app_data(web::Data::new(task_service.clone()))
            // Configuración de JSON
            .app_data(web::JsonConfig::default()
                .error_handler(|err, _req| {
                    actix_web::error::InternalError::from_response(
                        err,
                        actix_web::HttpResponse::BadRequest()
                            .content_type("application/json")
                            .body(format!(r#"{{"error":"{}", "type":"validation"}}"#, err))
                    )
                    .into()
                })
            )
            // Rutas de la API
            .service(
                web::scope("/api/v1")
                    .route("/tasks", web::get().to(task_handler::list_tasks))
                    .route("/tasks", web::post().to(task_handler::create_task))
                    .route("/tasks/{id}", web::get().to(task_handler::get_task))
                    .route("/tasks/{id}", web::put().to(task_handler::update_task))
            )
            // Endpoint de salud
            .route("/health", web::get().to(|| {
                async {
                    actix_web::HttpResponse::Ok()
                        .content_type("application/json")
                        .body(r#"{"status":"healthy","service":"task-manager"}"#)
                }
            }))
    })
    .bind(&bind_address)?
    .run()
    .await
}
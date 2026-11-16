//! Definición de la especificación OpenAPI/Swagger para la API de tareas.
//! Este módulo proporciona la documentación automática de la API usando Utoipa.

use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use crate::domain::models::{Task, CreateTaskDto, UpdateTaskDto, TaskStatus};
use crate::domain::errors::DomainError;

/// Respuesta de error para operaciones de tareas.
/// Define la estructura de los errores devueltos por la API.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TaskErrorResponse {
    /// Código de error interno.
    pub code: String,
    /// Mensaje descriptivo del error.
    pub message: String,
    /// Descripción detallada del error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl TaskErrorResponse {
    /// Crea una nueva respuesta de error.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Crea una nueva respuesta de error con detalles.
    pub fn with_details(code: impl Into<String>, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details.into()),
        }
    }

    /// Crea una respuesta de error de validación.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new("VALIDATION_ERROR", message)
    }

    /// Crea una respuesta de error de recurso no encontrado.
    pub fn not_found(id: i64) -> Self {
        Self::new("NOT_FOUND", format!("Tarea con ID {} no encontrada", id))
    }

    /// Crea una respuesta de error interna del servidor.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }
}

impl From<DomainError> for TaskErrorResponse {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::ValidationError(msg) => Self::validation(msg),
            DomainError::NotFound(id) => Self::not_found(id),
            DomainError::Internal(msg) => Self::internal(msg),
            DomainError::Database(msg) => Self::internal(format!("Error de base de datos: {}", msg)),
        }
    }
}

/// Documentación OpenAPI para la API de gestión de tareas.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "API de Gestión de Tareas",
        version = "1.0.0",
        description = "API REST para gestionar tareas en un entorno de proyecto de software. Permite crear, listar y actualizar tareas.",
        contact(
            name = "Soporte",
            email = "soporte@ejemplo.com"
        )
    ),
    servers(
        (url = "http://127.0.0.1:8080", description = "Servidor de desarrollo local")
    ),
    components(
        schemas(
            Task,
            CreateTaskDto,
            UpdateTaskDto,
            TaskStatus,
            TaskErrorResponse
        ),
        security_schemes(
            ApiKeyAuth = SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key")))
        )
    ),
    tags(
        (name = "tareas", description = "Operaciones relacionadas con la gestión de tareas")
    )
)]
pub struct ApiDoc;

/// Genera el documento OpenAPI completo como string JSON.
/// Este método se utiliza para servir la especificación en el endpoint /openapi.json.
impl ApiDoc {
    /// Genera la especificación OpenAPI en formato JSON.
    pub fn openapi() -> utoipa::openapi::OpenApi {
        Self::openapi_with_context()
    }

    /// Genera la especificación con contexto personalizado.
    fn openapi_with_context() -> utoipa::openapi::OpenApi {
        let mut openapi = <ApiDoc as utoipa::OpenApi>::openapi();
        
        if let Some(paths) = openapi.paths_mut().as_mut() {
            paths.add_path(
                "/api/health",
                utoipa::openapi::PathItem::new(
                    utoipa::openapi::PathItemType::Get,
                    vec![
                        utoipa::openapi::Operation {
                            summary: Some("Verificar estado de la API".to_string()),
                            description: Some("Endpoint de health check para verificar que la API está funcionando.".to_string()),
                            tags: Some(vec!["salud".to_string()]),
                            responses: Some(utoipa::openapi::Responses::default()
                                .add("200", utoipa::openapi::Response::new(
                                    "OK - La API está funcionando correctamente"
                                ))
                            ),
                            ..Default::default()
                        }
                    ]
                )
            );
        }
        
        openapi
    }
}

/// Configuración de seguridad para la API.
/// Define el esquema de autenticación por API Key.
pub fn security_scheme() -> utoipa::openapi::security::SecurityScheme {
    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key")))
}

/// Genera el JSON de la especificación OpenAPI.
/// Útil para servir en endpoints o generar documentación.
pub fn generate_openapi_json() -> String {
    let openapi = ApiDoc::openapi();
    serde_json::to_string_pretty(&openapi).unwrap_or_else(|e| {
        format!(r#"{{"error": "Error al generar OpenAPI: {}", "details": "{}"}}"#, e)
    })
}

/// Extension trait para agregar paths de forma dinámica a la especificación.
pub trait OpenApiExt {
    fn add_task_paths(&mut self);
}

impl OpenApiExt for utoipa::openapi::OpenApi {
    /// Agrega los paths específicos de tareas a la especificación OpenAPI.
    fn add_task_paths(&mut self) {
        if let Some(paths) = self.paths_mut().as_mut() {
            paths.add_path(
                "/tasks",
                utoipa::openapi::PathItem::new(
                    utoipa::openapi::PathItemType::Get,
                    vec![
                        utoipa::openapi::Operation {
                            summary: Some("Listar todas las tareas".to_string()),
                            description: Some("Retorna una lista de todas las tareas existentes en el sistema.".to_string()),
                            tags: Some(vec!["tareas".to_string()]),
                            security: Some(vec![utoipa::openapi::security::SecurityRequirement::new(
                                "ApiKeyAuth",
                                vec![]
                            )]),
                            responses: Some(utoipa::openapi::Responses::default()
                                .add("200", utoipa::openapi::Response::new(
                                    "Lista de tareas obtenida exitosamente"
                                ).add_content(
                                    "application/json",
                                    utoipa::openapi::Content::new(
                                        utoipa::openapi::Schema::from(
                                            utoipa::openapi::Object::new()
                                                .schema_type(utoipa::openapi::SchemaType::Array)
                                        )
                                    )
                                ))
                                .add("500", utoipa::openapi::Response::new(
                                    "Error interno del servidor"
                                ))
                            ),
                            ..Default::default()
                        }
                    ]
                )
            );

            paths.add_path(
                "/tasks",
                utoipa::openapi::PathItem::new(
                    utoipa::openapi::PathItemType::Post,
                    vec![
                        utoipa::openapi::Operation {
                            summary: Some("Crear una nueva tarea".to_string()),
                            description: Some("Crea una nueva tarea con los datos proporcionados.".to_string()),
                            tags: Some(vec!["tareas".to_string()]),
                            security: Some(vec![utoipa::openapi::security::SecurityRequirement::new(
                                "ApiKeyAuth",
                                vec![]
                            )]),
                            request_body: Some(utoipa::openapi::RequestBody::new(
                                Some(utoipa::openapi::Content::new(
                                    utoipa::openapi::Schema::from(
                                        utoipa::openapi::Object::new()
                                            .schema_type(utoipa::openapi::SchemaType::Object)
                                    )
                                )),
                                "application/json"
                            )),
                            responses: Some(utoipa::openapi::Responses::default()
                                .add("201", utoipa::openapi::Response::new(
                                    "Tarea creada exitosamente"
                                ))
                                .add("400", utoipa::openapi::Response::new(
                                    "Datos de entrada inválidos"
                                ))
                                .add("500", utoipa::openapi::Response::new(
                                    "Error interno del servidor"
                                ))
                            ),
                            ..Default::default()
                        }
                    ]
                )
            );

            paths.add_path(
                "/tasks/{id}",
                utoipa::openapi::PathItem::new(
                    utoipa::openapi::PathItemType::Get,
                    vec![
                        utoipa::openapi::Operation {
                            summary: Some("Obtener una tarea por ID".to_string()),
                            description: Some("Retorna los detalles de una tarea específica.".to_string()),
                            tags: Some(vec!["tareas".to_string()]),
                            security: Some(vec![utoipa::openapi::security::SecurityRequirement::new(
                                "ApiKeyAuth",
                                vec![]
                            )]),
                            parameters: Some(vec![
                                utoipa::openapi::Parameter::new(
                                    "id",
                                    utoipa::openapi::ParameterIn::Path,
                                ).description(Some("ID de la tarea a obtener"))
                            ]),
                            responses: Some(utoipa::openapi::Responses::default()
                                .add("200", utoipa::openapi::Response::new(
                                    "Tarea encontrada"
                                ))
                                .add("404", utoipa::openapi::Response::new(
                                    "Tarea no encontrada"
                                ))
                                .add("500", utoipa::openapi::Response::new(
                                    "Error interno del servidor"
                                ))
                            ),
                            ..Default::default()
                        }
                    ]
                )
            );

            paths.add_path(
                "/tasks/{id}",
                utoipa::openapi::PathItem::new(
                    utoipa::openapi::PathItemType::Put,
                    vec![
                        utoipa::openapi::Operation {
                            summary: Some("Actualizar una tarea".to_string()),
                            description: Some("Actualiza los datos de una tarea existente.".to_string()),
                            tags: Some(vec!["tareas".to_string()]),
                            security: Some(vec![utoipa::openapi::security::SecurityRequirement::new(
                                "ApiKeyAuth",
                                vec![]
                            )]),
                            parameters: Some(vec![
                                utoipa::openapi::Parameter::new(
                                    "id",
                                    utoipa::openapi::ParameterIn::Path,
                                ).description(Some("ID de la tarea a actualizar"))
                            ]),
                            request_body: Some(utoipa::openapi::RequestBody::new(
                                Some(utoipa::openapi::Content::new(
                                    utoipa::openapi::Schema::from(
                                        utoipa::openapi::Object::new()
                                            .schema_type(utoipa::openapi::SchemaType::Object)
                                    )
                                )),
                                "application/json"
                            )),
                            responses: Some(utoipa::openapi::Responses::default()
                                .add("200", utoipa::openapi::Response::new(
                                    "Tarea actualizada exitosamente"
                                ))
                                .add("400", utoipa::openapi::Response::new(
                                    "Datos de entrada inválidos"
                                ))
                                .add("404", utoipa::openapi::Response::new(
                                    "Tarea no encontrada"
                                ))
                                .add("500", utoipa::openapi::Response::new(
                                    "Error interno del servidor"
                                ))
                            ),
                            ..Default::default()
                        }
                    ]
                )
            );
        }
    }
}
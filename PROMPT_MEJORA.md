# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener el proyecto completo y arrancable. Guarda los archivos que te devuelva en la
carpeta del repo, respetando exactamente las rutas.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Superficie de practica — NO resuelvas

Estos archivos SON el ejercicio de la persona. No los implementes; deja stubs.

- `src/domain/infrastructure/handlers.rs` — La fase "Definición y Creación de Endpoints" se implementa aqui: este archivo es el ejercicio.
- `src/domain/errors.rs` — La fase "Validación y Manejo de Errores" se implementa aqui: este archivo es el ejercicio.

## Como saber que terminaste

```bash
cargo check
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Perfil
Chapter Backend, Especialidad Desarrollador, Tecnología Rust, Senior L2

### Brecha de conocimiento
Construir una API REST en Rust con Actix Web para gestionar tareas, Implementa endpoints REST en Rust con Actix Web para crear, listar y actualizar tareas

### Misión / candidato
Exponer la gestion de tareas por HTTP

### Reto
- Tema: API REST de tareas con Rust y Actix
- Seniority: senior-l2
- Tipo: practical
- Título: Gestión de Tareas con API REST en Rust
- Tiempo estimado: 10 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Definición y Creación de Endpoints — objetivo: Definir y crear los endpoints REST necesarios para crear, listar y actualizar tareas. — entregable (NO resolver): Endpoints REST funcionales para crear, listar y actualizar tareas.
- Fase 2: Validación y Manejo de Errores — objetivo: Implementar validaciones y manejo de errores en los endpoints. — entregable (NO resolver): Endpoints REST con validaciones y manejo de errores implementados.
- Fase 3: Optimización y Documentación — objetivo: Optimizar el rendimiento de los endpoints y documentar la API. — entregable (NO resolver): Endpoints REST optimizados y documentación de la API utilizando OpenAPI.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (entrega de archivos).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Entrega de archivos
Entregá cada archivo con su ruta completa, respetando exactamente la estructura de rutas
indicada por los marcadores: un bloque por archivo, precedido por su ruta. No generes ZIPs
ni archivos adjuntos.
La entrega debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: Entregá los archivos completos al finalizar. No preguntes si el usuario
quiere recibirlos. Simplemente entregalos; No debes desplegar en el chat el resumen de lo que arreglaste, solo entregá los archivos.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del último archivo entregado
No preguntes si el usuario quiere los archivos: simplemente entregalos siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: Cargo.toml ===
[package]
name = "actix-web-skeleton"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.19.0"
anyhow = "1.0.104"
async-trait = "0.1.92"
dotenv = "0.15.0"
serde = { version = "1.0.229", features = ["derive"] }
sqlx = { version = "0.9.0", features = ["runtime-tokio", "macros"] }
thiserror = "1.0.69"
validator = "0.21.0"
actix-web-validator = "5.0.1"
chrono = { version = "0.4.45", features = ["serde"] }
log = "0.4.35"
utoipa = "4.2.3"

[dev-dependencies]
actix-rt = "2.15.0"

// === ARCHIVO: src/contracts/types.rs ===
// Sample contract types
pub trait DomainContract {
    fn validate(&self) -> bool;
}

// === ARCHIVO: src/domain/mod.rs ===
pub mod models;
pub mod errors;
pub mod application;
pub mod infrastructure;
pub mod config;

// === ARCHIVO: src/contracts/mod.rs ===
// Contracts root module
pub mod types;

// === ARCHIVO: src/domain/models.rs ===
//! Modelos de dominio para la gestión de tareas.

use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::{DateTime, Utc};

/// Representa una tarea en el sistema.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Task {
    /// Identificador único de la tarea.
    pub id: Option<i64>,

    /// Título de la tarea.
    #[validate(length(min = 1, max = 100, message = "El título debe tener entre 1 y 100 caracteres"))]
    pub title: String,

    /// Descripción detallada de la tarea.
    #[validate(length(max = 500, message = "La descripción no puede exceder 500 caracteres"))]
    pub description: Option<String>,

    /// Estado actual de la tarea.
    pub status: TaskStatus,

    /// Fecha de creación de la tarea.
    pub created_at: DateTime<Utc>,

    /// Fecha de última actualización de la tarea.
    pub updated_at: DateTime<Utc>,
}

/// Estados posibles de una tarea.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// Tarea pendiente de realizar.
    Pending,
    /// Tarea en progreso.
    InProgress,
    /// Tarea completada.
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

/// DTO para la creación de una nueva tarea.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTaskDto {
    /// Título de la tarea.
    #[validate(length(min = 1, max = 100, message = "El título debe tener entre 1 y 100 caracteres"))]
    pub title: String,

    /// Descripción detallada de la tarea.
    #[validate(length(max = 500, message = "La descripción no puede exceder 500 caracteres"))]
    pub description: Option<String>,
}

/// DTO para la actualización de una tarea existente.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTaskDto {
    /// Título de la tarea.
    #[validate(length(min = 1, max = 100, message = "El título debe tener entre 1 y 100 caracteres"))]
    pub title: Option<String>,

    /// Descripción detallada de la tarea.
    #[validate(length(max = 500, message = "La descripción no puede exceder 500 caracteres"))]
    pub description: Option<String>,

    /// Estado actual de la tarea.
    pub status: Option<TaskStatus>,
}

impl Task {
    /// Crea una nueva tarea con los datos proporcionados.
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Task {
            id: None,
            title,
            description,
            status: TaskStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Actualiza los campos de la tarea con los datos proporcionados.
    pub fn update(&mut self, update_dto: UpdateTaskDto) {
        if let Some(title) = update_dto.title {
            self.title = title;
        }
        if let Some(description) = update_dto.description {
            self.description = Some(description);
        }
        if let Some(status) = update_dto.status {
            self.status = status;
        }
        self.updated_at = Utc::now();
    }
}

// === ARCHIVO: src/domain/errors.rs ===
// TODO(reto): archivo stubbeado; implementa los cuerpos.
//! Errores de dominio y sus representaciones HTTP/API.
//! TODO(reto): Implementar los errores específicos del dominio y su conversión a respuestas HTTP.

use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

/// Errores del dominio de tareas.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Tarea no encontrada")]
    TaskNotFound,

    #[error("Error interno del servidor")]
    InternalServerError,
}

/// Respuesta de error para la API.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
      todo!("TODO(reto): error_response")
    }
}

// === ARCHIVO: src/domain/application/services.rs ===
//! Servicios de aplicación para la gestión de tareas.

use crate::domain::models::{Task, CreateTaskDto, UpdateTaskDto};
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use chrono::Utc;

/// Trait que define las operaciones del repositorio de tareas.
#[async_trait]
pub trait TaskRepository: Send + Sync {
    /// Guarda una nueva tarea en el repositorio.
    async fn save(&self, task: &mut Task) -> Result<(), DomainError>;

    /// Obtiene una tarea por su ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<Task>, DomainError>;

    /// Lista todas las tareas.
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;

    /// Actualiza una tarea existente.
    async fn update(&self, task: &Task) -> Result<(), DomainError>;
}

/// Servicio de aplicación para la gestión de tareas.
pub struct TaskService<T: TaskRepository> {
    repository: T,
}

impl<T: TaskRepository> TaskService<T> {
    /// Crea una nueva instancia del servicio de tareas.
    pub fn new(repository: T) -> Self {
        TaskService { repository }
    }

    /// Crea una nueva tarea.
    pub async fn create_task(&self, create_dto: CreateTaskDto) -> Result<Task, DomainError> {
        let mut task = Task::new(create_dto.title, create_dto.description);
        self.repository.save(&mut task).await?;
        Ok(task)
    }

    /// Obtiene una tarea por su ID.
    pub async fn get_task(&self, id: i64) -> Result<Task, DomainError> {
        match self.repository.find_by_id(id).await? {
            Some(task) => Ok(task),
            None => Err(DomainError::TaskNotFound),
        }
    }

    /// Lista todas las tareas.
    pub async fn list_tasks(&self) -> Result<Vec<Task>, DomainError> {
        self.repository.find_all().await
    }

    /// Actualiza una tarea existente.
    pub async fn update_task(&self, id: i64, update_dto: UpdateTaskDto) -> Result<Task, DomainError> {
        let mut task = self.get_task(id).await?;
        task.update(update_dto);
        self.repository.update(&task).await?;
        Ok(task)
    }
}

// === ARCHIVO: src/domain/infrastructure/repositories.rs ===
//! Implementación del repositorio de tareas para persistencia.

use crate::domain::models::Task;
use crate::domain::errors::DomainError;
use crate::domain::application::services::TaskRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;

/// Implementación del repositorio de tareas usando PostgreSQL.
pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    /// Crea una nueva instancia del repositorio de tareas.
    pub fn new(pool: PgPool) -> Self {
        PgTaskRepository { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn save(&self, task: &mut Task) -> Result<(), DomainError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO tasks (title, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            task.title,
            task.description,
            task.status,
            task.created_at,
            task.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        task.id = Some(result.id);
        Ok(())
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Task>, DomainError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, description, status as "status: _", created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn find_all(&self) -> Result<Vec<Task>, DomainError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, description, status as "status: _", created_at, updated_at
            FROM tasks
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn update(&self, task: &Task) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE tasks
            SET title = $1, description = $2, status = $3, updated_at = $4
            WHERE id = $5
            "#,
            task.title,
            task.description,
            task.status,
            Utc::now(),
            task.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// === ARCHIVO: src/domain/infrastructure/handlers.rs ===
//! Manejadores HTTP para las rutas de la API.
//! TODO(reto): Implementar los handlers para los endpoints de tareas.

use actix_web::{web, HttpResponse};
use crate::domain::models::{CreateTaskDto, UpdateTaskDto};
use crate::domain::application::services::TaskService;
use crate::domain::errors::DomainError;
use actix_web_validator::Json;

/// Handler para crear una nueva tarea.
pub async fn create_task(
    task_service: web::Data<TaskService<impl TaskRepository>>,
    task_dto: Json<CreateTaskDto>,
) -> Result<HttpResponse, DomainError> {
    // TODO(reto): Implementar la lógica para crear una tarea.
    Ok(HttpResponse::Ok().finish())
}

/// Handler para listar todas las tareas.
pub async fn list_tasks(
    task_service: web::Data<TaskService<impl TaskRepository>>,
) -> Result<HttpResponse, DomainError> {
    // TODO(reto): Implementar la lógica para listar tareas.
    Ok(HttpResponse::Ok().finish())
}

/// Handler para obtener una tarea por su ID.
pub async fn get_task(
    task_service: web::Data<TaskService<impl TaskRepository>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    // TODO(reto): Implementar la lógica para obtener una tarea.
    Ok(HttpResponse::Ok().finish())
}

/// Handler para actualizar una tarea existente.
pub async fn update_task(
    task_service: web::Data<TaskService<impl TaskRepository>>,
    id: web::Path<i64>,
    task_dto: Json<UpdateTaskDto>,
) -> Result<HttpResponse, DomainError> {
    // TODO(reto): Implementar la lógica para actualizar una tarea.
    Ok(HttpResponse::Ok().finish())
}

// === ARCHIVO: src/domain/config/mod.rs ===
//! Configuración de la aplicación.

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Configuración de la aplicación.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// URL de conexión a la base de datos.
    pub database_url: String,
    /// Puerto en el que escucha la aplicación.
    pub port: u16,
}

impl AppConfig {
    /// Carga la configuración desde las variables de entorno.
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")?,
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
        })
    }

    /// Crea un pool de conexiones a la base de datos.
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
    }
}

// === ARCHIVO: src/domain/infrastructure/openapi.rs ===
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

// === ARCHIVO: src/main.rs ===
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

// === ARCHIVO: src/domain/application/mod.rs ===
pub mod services;


// === ARCHIVO: src/domain/infrastructure/mod.rs ===
pub mod handlers;
pub mod openapi;
pub mod repositories;

```

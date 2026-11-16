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
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
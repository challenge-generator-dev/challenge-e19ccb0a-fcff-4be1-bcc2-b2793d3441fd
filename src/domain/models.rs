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
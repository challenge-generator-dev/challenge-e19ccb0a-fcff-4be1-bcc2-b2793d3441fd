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
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
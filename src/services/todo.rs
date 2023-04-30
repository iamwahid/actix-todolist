use crate::common::TitleField;
use chrono::{NaiveDateTime, Utc};
use sqlx::MySqlPool;

pub fn default_as_true() -> bool {
    true
}

pub fn default_as_very_high() -> String {
    "very-high".into()
}

#[derive(serde::Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    priority: Option<String>,
    activity_group_id: i32,
    #[serde(default = "default_as_true")]
    is_active: Option<bool>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
}

#[allow(dead_code)]
pub struct TodoTable {
    id: i32,
    title: String,
    priority: String,
    activity_group_id: i32,
    is_active: i8,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    r#deleted_at: Option<NaiveDateTime>,
}

pub struct NewTodo {
    pub title: TitleField,
    pub activity_group_id: i32,
    pub is_active: Option<bool>,
    pub priority: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub activity_group_id: Option<i32>,
    pub is_active: Option<bool>,
    pub priority: Option<String>,
}

pub async fn get_todos(
    activity_group_id: Option<i32>,
    pool: &MySqlPool,
) -> Result<Vec<Todo>, sqlx::Error> {
    let query = match activity_group_id {
        Some(activity_group_id) => sqlx::query_as!(
            TodoTable,
            "select * from todos where activity_group_id = ?",
            activity_group_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?,
        None => sqlx::query_as!(TodoTable, "select * from todos")
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?,
    };

    let mut todos = Vec::<Todo>::new();

    for q in query {
        todos.push(Todo {
            id: q.id,
            title: q.title,
            activity_group_id: q.activity_group_id,
            is_active: Some(q.is_active != 0),
            priority: Some(q.priority),
            created_at: q.created_at.to_string(),
            updated_at: match q.updated_at {
                Some(updated_at) => Some(updated_at.to_string()),
                None => None,
            },
        });
    }

    Ok(todos)
}

pub async fn get_todo_by_id(todo_id: i32, pool: &MySqlPool) -> Result<Todo, sqlx::Error> {
    let query = sqlx::query!("select * from todos where id = ?", todo_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(Todo {
        id: query.id,
        title: query.title,
        activity_group_id: query.activity_group_id,
        is_active: Some(query.is_active != 0),
        priority: Some(query.priority),
        created_at: query.created_at.to_string(),
        updated_at: match query.updated_at {
            Some(updated_at) => Some(updated_at.to_string()),
            None => None,
        },
    })
}

pub async fn update_todo_by_id(
    todo_id: i32,
    pool: &MySqlPool,
    form: &UpdateTodo,
) -> Result<Todo, sqlx::Error> {
    let current_todo = sqlx::query!("select * from todos  where id = ?", todo_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let _query = match &form.title {
        Some(title) => sqlx::query!(
            "update todos set title = ?, activity_group_id = ?, is_active = ?, priority = ?, updated_at = ? where id = ?",
            title.clone(),
            current_todo.activity_group_id,
            current_todo.is_active,
            current_todo.priority,
            Utc::now(),
            todo_id
        ),
        None => sqlx::query!(
            "update todos set updated_at = ? where id = ?",
            Utc::now(),
            todo_id
        )
    }
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let _query = match &form.activity_group_id {
        Some(activity_group_id) => sqlx::query!(
            "update todos set title = ?, activity_group_id = ?, is_active = ?, priority = ?, updated_at = ? where id = ?",
            current_todo.title,
            activity_group_id,
            current_todo.is_active,
            current_todo.priority,
            Utc::now(),
            todo_id
        ),
        None => sqlx::query!(
            "update todos set updated_at = ? where id = ?",
            Utc::now(),
            todo_id
        )
    }
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let _query = match &form.is_active {
        Some(is_active) => sqlx::query!(
            "update todos set title = ?, activity_group_id = ?, is_active = ?, priority = ?, updated_at = ? where id = ?",
            current_todo.title,
            current_todo.activity_group_id,
            is_active,
            current_todo.priority,
            Utc::now(),
            todo_id
        ),
        None => sqlx::query!(
            "update todos set updated_at = ? where id = ?",
            Utc::now(),
            todo_id
        )
    }
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let _query = match &form.priority {
        Some(priority) => sqlx::query!(
            "update todos set title = ?, activity_group_id = ?, is_active = ?, priority = ?, updated_at = ? where id = ?",
            current_todo.title,
            current_todo.activity_group_id,
            current_todo.is_active,
            priority,
            Utc::now(),
            todo_id
        ),
        None => sqlx::query!(
            "update todos set updated_at = ? where id = ?",
            Utc::now(),
            todo_id
        )
    }
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let record = sqlx::query!("select * from todos where id = ?", todo_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(Todo {
        id: record.id,
        title: record.title,
        activity_group_id: record.activity_group_id,
        is_active: Some(record.is_active != 0),
        priority: Some(record.priority),
        created_at: record.created_at.to_string(),
        updated_at: match record.updated_at {
            Some(updated_at) => Some(updated_at.to_string()),
            None => None,
        },
    })
}

pub async fn delete_todo_by_id(todo_id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let _record = sqlx::query!("select * from todos where id = ?", todo_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    sqlx::query!("delete from todos where id = ?", todo_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(())
}

pub async fn insert_todo(pool: &MySqlPool, form: &NewTodo) -> Result<Todo, sqlx::Error> {
    let utc_now = Utc::now();

    let query = sqlx::query!(
        r#"
        insert into todos (title, activity_group_id, is_active, priority, created_at, updated_at)
        values (?, ?, ?, ?, ?, ?)
        "#,
        form.title.inner_ref().clone(),
        form.activity_group_id,
        form.is_active,
        form.priority,
        utc_now.clone(),
        utc_now.clone()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(Todo {
        id: query.last_insert_id() as i32,
        title: form.title.inner_ref().into(),
        activity_group_id: form.activity_group_id.clone(),
        is_active: form.is_active,
        priority: form.priority.clone(),
        created_at: utc_now.to_string(),
        updated_at: Some(utc_now.to_string()),
    })
}

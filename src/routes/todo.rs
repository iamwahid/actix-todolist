use actix_web::{get, patch, post, delete, web, Responder, HttpRequest, HttpResponse};
use serde_json::{Map, Value};
use sqlx::MySqlPool;
use crate::common::{TitleField, Response, ResponseWithData};
use crate::services::{Todo, UpdateTodo, NewTodo};
use crate::services::{get_todos, insert_todo, get_todo_by_id, update_todo_by_id, delete_todo_by_id, default_as_true, default_as_very_high};

#[derive(serde::Deserialize)]
struct FormData {
    #[serde(default)]
    title: String,
    activity_group_id: Option<i32>,
    #[serde(default = "default_as_very_high")]
    priority: String,
    #[serde(default = "default_as_true")]
    is_active: bool,
}

#[derive(serde::Deserialize)]
struct FormUpdateData {
    title: Option<String>,
    activity_group_id: Option<i32>,
    priority: Option<String>,
    is_active: Option<bool>,
}

#[derive(serde::Deserialize)]
struct Params {
    activity_group_id: Option<i32>,
}


#[get("/todo-items")]
pub async fn todo_list(req: HttpRequest, params: web::Query<Params>, pool: web::Data<MySqlPool>) -> HttpResponse {
    match get_todos(params.activity_group_id, &pool).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Vec<Todo>> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/todo-items")]
pub async fn todo_create(form: web::Json<FormData>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let title = match TitleField::parse(form.0.title.clone()) {
        Ok(title) => title,
        Err(_) => {
            let error = Response{ status: "Bad Request".into(), message: "title cannot be null".into()};
            return HttpResponse::BadRequest().json(error);
        }
    };

    let activity_group_id = match form.0.activity_group_id {
        Some(activity_group_id) => activity_group_id,
        None => {
            let error = Response{ status: "Bad Request".into(), message: "activity_group_id cannot be null".into()};
            return HttpResponse::BadRequest().json(error);
        }
    };

    let activity = NewTodo {
        title,
        activity_group_id: activity_group_id,
        priority: Some(form.0.priority.clone()),
        is_active: Some(form.0.is_active),
    };

    match insert_todo(&pool, &activity).await
    {
        Ok(data) => HttpResponse::Created().json(ResponseWithData::<Todo> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/todo-items/{todo_id}")] // <- define path parameters
pub async fn todo_detail(path: web::Path<i32>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let todo_id = path.into_inner();
    match get_todo_by_id(todo_id, &pool).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Todo> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Todo with ID {} Not Found", todo_id)})
        }
    }
}

#[patch("/todo-items/{todo_id}")] // <- define path parameters
pub async fn todo_update(path: web::Path<i32>, form: web::Json<FormUpdateData>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let todo_id = path.into_inner();
    // let title = match TitleField::parse(form.0.title.clone()) {
    //     Ok(title) => title,
    //     Err(_) => {
    //         let error = Response{ status: "Bad Request".into(), message: "title cannot be null".into()};
    //         return HttpResponse::BadRequest().json(error);
    //     }
    // };

    let todo = UpdateTodo {
        title: form.0.title,
        activity_group_id: form.0.activity_group_id,
        is_active: form.0.is_active,
        priority: form.0.priority,
    };
    match update_todo_by_id(todo_id, &pool, &todo).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Todo> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Todo with ID {} Not Found", todo_id)})
        }
    }
}

#[delete("/todo-items/{todo_id}")] // <- define path parameters
pub async fn todo_destroy(path: web::Path<i32>, pool: web::Data<MySqlPool>) -> impl Responder {
    let todo_id = path.into_inner();
    match delete_todo_by_id(todo_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json(ResponseWithData::<Map<String, Value>> {
                status: "Success".into(),
                message: "Success".into(),
                data: Map::<String, Value>::new()
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Todo with ID {} Not Found", todo_id)})
        }
    }
}

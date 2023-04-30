use actix_web::{get, patch, post, delete, web, HttpRequest, HttpResponse};
use serde_json::{Map, Value};
use sqlx::MySqlPool;
use crate::common::{TitleField, Response, ResponseWithData};
use crate::services::{Activity, UpdateActivity, NewActivity};
use crate::services::{get_activities, insert_activity, get_activity_by_id, update_activity_by_id, delete_activity_by_id};


#[derive(serde::Deserialize)]
struct FormData {
    #[serde(default)]
    title: String,
    email: Option<String>
}

#[get("/activity-groups")]
pub async fn activity_list(_req: HttpRequest, pool: web::Data<MySqlPool>) -> HttpResponse {
    match get_activities(&pool).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Vec<Activity>> {
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

#[post("/activity-groups")]
pub async fn activity_create(form: web::Json<FormData>, pool: web::Data<MySqlPool>) -> HttpResponse {

    let title = match TitleField::parse(form.0.title.clone()) {
        Ok(title) => title,
        Err(_) => {
            let error = Response{ status: "Bad Request".into(), message: "title cannot be null".into()};
            return HttpResponse::BadRequest().json(error);
        }
    };

    let activity = NewActivity {
        title,
        email: form.0.email,
    };

    match insert_activity(&pool, &activity).await
    {
        Ok(data) => HttpResponse::Created().json(ResponseWithData::<Activity> {
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

#[get("/activity-groups/{activity_id}")] 
pub async fn activity_detail(path: web::Path<i32>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let activity_id = path.into_inner();
    match get_activity_by_id(activity_id, &pool).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Activity> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Activity with ID {} Not Found", activity_id)})
        }
    }
}

#[patch("/activity-groups/{activity_id}")] 
pub async fn activity_update(path: web::Path<i32>, form: web::Json<FormData>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let activity_id = path.into_inner();

    let title = match TitleField::parse(form.0.title.clone()) {
        Ok(title) => title,
        Err(_) => {
            let error = Response{ status: "Bad Request".into(), message: "title cannot be null".into()};
            return HttpResponse::BadRequest().json(error);
        }
    };

    let activity = UpdateActivity {
        title,
        email: form.0.email,
    };
    match update_activity_by_id(activity_id, &pool, &activity).await {
        Ok(data) => HttpResponse::Ok().json(ResponseWithData::<Activity> {
            status: "Success".into(),
            message: "Success".into(),
            data: data
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Activity with ID {} Not Found", activity_id)})
        }
    }
}


#[delete("/activity-groups/{activity_id}")] 
pub async fn activity_destroy(path: web::Path<i32>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let activity_id = path.into_inner();
    match delete_activity_by_id(activity_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json(ResponseWithData::<Map<String, Value>> {
                status: "Success".into(),
                message: "Success".into(),
                data: Map::<String, Value>::new()
        }),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::NotFound().json(Response{ status: "Not Found".into(), message: format!("Activity with ID {} Not Found", activity_id)})
        }
    }
}

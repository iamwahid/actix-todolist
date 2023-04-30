use chrono::Utc;
use sqlx::MySqlPool;
use crate::common::TitleField;

#[derive(serde::Serialize)]
pub struct Activity {
    id: i32,
    title: String,
    email: Option<String>,
    #[serde(rename="createdAt")]
    created_at: String,
    #[serde(rename="updatedAt")]
    updated_at: Option<String>
}


pub struct NewActivity {
    pub title: TitleField,
    pub email: Option<String>,
}

pub struct UpdateActivity {
    pub title: TitleField,
    pub email: Option<String>,
}

pub async fn get_activities(
    pool: &MySqlPool,
) -> Result<Vec<Activity>, sqlx::Error> {
    let query = sqlx::query!("select * from activities")
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    let mut activities = Vec::<Activity>::new();

    for q in query {
        activities.push(
            Activity { 
                id: q.id,
                title: q.title,
                email: q.email,
                created_at: q.created_at.to_string(),
                updated_at: match q.updated_at {
                    Some(updated_at) => Some(updated_at.to_string()),
                    None => None
                }
            }
        );
    }

    Ok(activities)
}

pub async fn get_activity_by_id(
    activity_id: i32,
    pool: &MySqlPool,
) -> Result<Activity, sqlx::Error> {
    let query = sqlx::query!("select * from activities where id = ?", activity_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(Activity {
        id: query.id,
        title: query.title,
        email: query.email,
        created_at: query.created_at.to_string(),
        updated_at: match query.updated_at {
            Some(updated_at) => Some(updated_at.to_string()),
            None => None
        }
    })
}

pub async fn update_activity_by_id(
    activity_id: i32,
    pool: &MySqlPool,
    form: &UpdateActivity,
) -> Result<Activity, sqlx::Error> {
    let query = match &form.email {
        Some(email) => sqlx::query!(
            "update activities set title = ?, email = ? where id = ?",
            form.title.inner_ref().clone(),
            email.clone(),
            activity_id
        ),
        None => sqlx::query!(
            "update activities set title = ? where id = ?",
            form.title.inner_ref().clone(),
            activity_id
        )
    };

    let _query = query
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    let record = sqlx::query!(
        "select * from activities where id = ?",
        activity_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(Activity {
        id: record.id,
        title: record.title,
        email: record.email,
        created_at: record.created_at.to_string(),
        updated_at: match record.updated_at {
            Some(updated_at) => Some(updated_at.to_string()),
            None => None
        }
    })
}

pub async fn delete_activity_by_id(
    activity_id: i32,
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    let _record = sqlx::query!(
        "select * from activities where id = ?",
        activity_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    sqlx::query!(
        "delete from activities where id = ?",
        activity_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

pub async fn insert_activity(
    pool: &MySqlPool,
    form: &NewActivity,
) -> Result<Activity, sqlx::Error> {
    let utc_now = Utc::now();
    
    let query = sqlx::query!(
        r#"
        insert into activities (title, email, created_at, updated_at)
        values (?, ?, ?, ?)
        "#, 
        form.title.inner_ref().clone(),
        form.email.clone(),
        utc_now.clone(),
        utc_now.clone()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(Activity {
        id: query.last_insert_id() as i32,
        title: form.title.inner_ref().into(),
        email: form.email.clone(),
        created_at: utc_now.to_string(),
        updated_at: Some(utc_now.to_string())
    })
}
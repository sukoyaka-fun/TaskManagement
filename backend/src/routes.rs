use actix_web::{get, post, HttpResponse, web, error};
use deadpool_postgres::Pool;

use crate::user::user;
use crate::task::task;

#[get("/users")]
// ユーザー一覧の取得
async fn list_users(pool: web::Data<Pool>) -> Result<HttpResponse, error::Error> {
    let client = pool.get().await.map_err(|err| {
        log::debug!("unable to get postgres client: {:?}", err);
        error::ErrorInternalServerError("unable to get postgres client")
    })?;

    let users = user::User::all(&**client).await.map_err(|err| {
        log::debug!("unable to fetch users: {:?}", err);
        error::ErrorInternalServerError("unable to fetch users")
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("/tasks")]
// タスクの作成
async fn create_task(task_info: web::Json<task::Task>, pool: web::Data<Pool>) -> Result<HttpResponse, error::Error> {
    let task_data = task_info.into_inner(); // JSONからTask構造体に変換します

    let client = pool.get().await.map_err(|err| {
        log::debug!("unable to get postgres client: {:?}", err);
        error::ErrorInternalServerError("unable to get postgres client")
    })?;

    // 新しいタスクを挿入するクエリを準備します
    let query = "INSERT INTO tasks (name, description, status, user_id) VALUES ($1, $2, $3, $4) RETURNING id";
    
    // クエリを実行し、新しいタスクのIDを取得します
    let task_id: i32 = client.query_one(query, &[&task_data.name, &task_data.description, &task_data.status, &task_data.user_id])
        .await
        .map_err(|err| {
            log::debug!("unable to execute query: {:?}", err);
            error::ErrorInternalServerError("unable to execute query")
        })?
        .get(0);

    // 新しいタスクのIDを含むレスポンスを返します
    Ok(HttpResponse::Created().json(task_id))

}

#[get("/tasks")]
// タスク一覧の取得
async fn list_tasks(pool: web::Data<Pool>) -> Result<HttpResponse, error::Error> {
    let client = pool.get().await.map_err(|err| {
        log::debug!("unable to get postgres client: {:?}", err);
        error::ErrorInternalServerError("unable to get postgres client")
    })?;

    let tasks = task::Task::all(&**client).await.map_err(|err| {
        log::debug!("unable to fetch tasks: {:?}", err);
        error::ErrorInternalServerError("unable to fetch tasks")
    })?;

    Ok(HttpResponse::Ok().json(tasks))
}
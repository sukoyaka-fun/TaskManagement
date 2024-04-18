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
// async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
//     let client = match pool.get().await {
//         Ok(client) => client,
//         Err(err) => {
//             log::debug!("unable to get postgres client: {:?}", err);
//             return HttpResponse::InternalServerError().json("unable to get postgres client");
//         }
//     };
//     match user::User::all(&**client).await {
//         Ok(list) => HttpResponse::Ok().json(list),
//         Err(err) => {
//             log::debug!("unable to fetch users: {:?}", err);
//             return HttpResponse::InternalServerError().json("unable to fetch users");
//         }
//     }
// }

// #[post("/tasks")]
// // // タスクの作成
// async fn create_task(task_info: web::Json<task::Task>, pool: web::Data<Pool>) -> Result<HttpResponse> {
//     let task_data = task_info.into_inner(); // JSONからTask構造体に変換します

//     // データベース接続を取得します
//     let conn = pool.get().await.map_err(|e| {
//         // エラーハンドリングを行います
//         HttpResponse::InternalServerError().body(format!("Database connection error: {}", e))
//     })?;

//     // ここでトランザクションを開始し、タスクと依存関係をデータベースに追加します
//     let result = conn.transaction(|conn| {
//         // 以下にトランザクション内の処理を記述します
//         // タスクをデータベースに追加するロジック
//         // 依存関係をデータベースに追加するロジック
//         // など...

//         Ok(()) // トランザクションが正常に完了した場合はOk(())
//     });

//     // トランザクションの実行結果をチェックし、レスポンスを返します
//     match result {
//         Ok(_) => Ok(HttpResponse::Ok().finish()), // 成功した場合はHTTP 200を返します
//         Err(e) => {
//             // エラーハンドリングを行います
//             eprintln!("Transaction error: {}", e);
//             Ok(HttpResponse::InternalServerError().finish())
//         }
//     }
// }
use tokio_postgres::{Error, GenericClient, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Task {
    task_id: i32,
    task_name: String,
    description: String,
    status: String,
    user_id: i32,
    preceding_task: Option<i32>,
}

// impl From<Row> for User {
//     fn from(row: Row) -> Self {
//         Self {
//             id: row.get(0),
//             login: row.get(1),
//         }
//     }
// }

// impl User {
//     pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
//         let stmt = client.prepare("SELECT id, login FROM users").await?;
//         let rows = client.query(&stmt, &[]).await?;

//         Ok(rows.into_iter().map(User::from).collect())
//     }
// }

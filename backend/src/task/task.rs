use tokio_postgres::{Error, GenericClient, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub status: String,
    pub user_id: i32,
    // preceding_task: Option<i32>,
}

impl From<Row> for Task {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            status: row.get(3),
            user_id: row.get(4),
        }
    }
}

impl Task {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Task>, Error> {
        let stmt = client.prepare("SELECT id, name ,description, status, user_id FROM tasks").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(Task::from).collect())
    }
}

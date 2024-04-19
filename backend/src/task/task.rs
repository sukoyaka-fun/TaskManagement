use tokio_postgres::{Error, GenericClient, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    // pub id: i32,
    pub name: String,
    pub description: String,
    pub status: String,
    pub user_id: i32,
    // preceding_task: Option<i32>,
}
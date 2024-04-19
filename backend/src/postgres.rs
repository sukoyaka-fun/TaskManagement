use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

const SCRIPTS_UP: [(&str, &str); 2] = [
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_up.sql"),
    ),
    (
        "0002_create-tasks",
        include_str!("../migrations/0002_create-tasks_up.sql"),
    )
    // (
    //     "0003_create-dependencis",
    //     include_str!("../migrations/0003_create-dependencies_up.sql"),
    // ),
    // (
    //     "0004_create-categories",
    //     include_str!("../migrations/0004_create-categories_up.sql"),
    // ),
    // (
    //     "0005_create-task_categories",
    //     include_str!("../migrations/0005_create-task_categories_up.sql"),
    // )
];

const SCRIPTS_DOWN: [(&str, &str); 5] = [
    (
        "0005_create-task_categories",
        include_str!("../migrations/0005_create-task_categories_down.sql"),
    ),
    (
        "0004_create-categories",
        include_str!("../migrations/0004_create-categories_down.sql"),
    ),
    (
        "0003_create-dependencis",
        include_str!("../migrations/0003_create-dependencies_down.sql"),
    ),
    (
        "0002_create-tasks",
        include_str!("../migrations/0002_create-tasks_down.sql"),
    ),
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_down.sql"),
    )
];

fn create_config() -> Config {
    let mut cfg = Config::new();
    if let Ok(host) = std::env::var("PG_HOST") {
        cfg.host = Some(host);
    }
    if let Ok(dbname) = std::env::var("PG_DBNAME") {
        cfg.dbname = Some(dbname);
    }
    if let Ok(user) = std::env::var("PG_USER") {
        cfg.user = Some(user);
    }
    if let Ok(password) = std::env::var("PG_PASSWORD") {
        cfg.password = Some(password);
    }
    cfg
}

pub fn create_pool() -> Pool {
    create_config()
        .create_pool(NoTls)
        .expect("couldn't create postgres pool")
}

pub async fn migrate_up(pool: &Pool) {
    let mut client = pool.get().await.expect("couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &SCRIPTS_UP)
        .await
        .expect("couldn't run migrations");
}

pub async fn migrate_down(pool: &Pool) { // テーブルを削除するマイグレーション
    let client = pool.get().await.expect("couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .down(& **client, &SCRIPTS_DOWN)
        .await
        .expect("couldn't run migrations");
}
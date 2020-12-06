use sqlx::{MySql, Pool, mysql, Row};
use std::time::{Duration, SystemTime};

pub async fn getConn() -> std::io::Result<Pool<MySql>> {
    Ok(mysql::MySqlPool::connect(&*format!("mysql://{}:{}@{}/{}", "root", "", "localhost:3306", "aoc-leaderboard-bot")).await.expect("Error while creating connection"))
}

pub async fn get_dev_schuppen_request_permission() -> bool {
    let conn = getConn().await.expect("Cannot connect to database");
    let last_time: i64 = sqlx::query("SELECT * FROM boards WHERE code='1047779-613e87db';")
        .fetch_one(&conn).await.unwrap().get(3);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    conn.close();
    if (now - (last_time as u64)) < 900 {
        false
    } else {
        true
    }
}

pub async fn set_devschuppen_requesttime(now: u64) {
    let conn = getConn().await.expect("Cannot connect to database");
    sqlx::query(format!("UPDATE `boards` SET `last_requested` = '{}' WHERE `boards`.`ID` = 1;", now).as_str()).execute(&conn).await;
    conn.close();
}
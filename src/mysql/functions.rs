use sqlx::{MySql, Pool, mysql, Row};

pub async fn getConn() -> std::io::Result<Pool<MySql>> {
    Ok(mysql::MySqlPool::connect(&*format!("mysql://{}:{}@{}/{}", "root", "", "localhost:3306", "aoc-leaderboard-bot")).await.expect("Error while creating connection"))
}

pub async fn get_dev_schuppen_request_permission() {
    let conn = getConn().await.expect("Cannot connect to database");
    let time: String = sqlx::query("SELECT * FROM boards WHERE code='1047779-613e87db';")
        .fetch_one(&conn).await.unwrap().get(3);
    println!("time: {}", time);

}
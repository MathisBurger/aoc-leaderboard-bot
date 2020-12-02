use reqwest;
use serde_json;
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, COOKIE};


pub struct user {
    pub(crate) name: String,
    pub(crate) coins: i32,
    pub(crate) stars: i32
}

pub async fn call_api() -> Vec<user>{
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap());
    headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert(COOKIE, "_ga=GA1.2.1673963742.1606845860; _gid=GA1.2.1296669231.1606845860; session=53616c7465645f5f59b1a6f0fa042c3bebbc066fee46fabb3ca305f9775ae1f62323165013e95dcff6ade772bd420f1b".parse().unwrap());
    let request_url = "https://adventofcode.com/2020/leaderboard/private/view/1047779.json";
    let res = reqwest::Client::new()
        .get(request_url)
        .headers(headers)
        .send().await.unwrap();
    let text = res.text().await.unwrap();
    let data: serde_json::Value = serde_json::from_str(text.as_str()).expect("Cannot parse json");
    let event = data["event"].as_str().unwrap().to_string();
    let mut user_info: Vec<user> = vec![];
    let user = data["members"].as_object().unwrap();
    for (k, v) in user.iter() {
        user_info.push(user {
            name: v["name"].as_str().unwrap().to_string(),
            coins: v["local_score"].as_i64().unwrap().as_i32(),
            stars: v["stars"].as_i64().unwrap().as_i32()
        });
    }
    user_info

}
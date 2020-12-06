use reqwest;
use serde_json;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, COOKIE};

#[derive(Deserialize, Serialize, Debug)]
pub struct user {
    pub(crate) name: String,
    pub(crate) coins: String,
    pub(crate) stars: String
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
    headers.insert(COOKIE, "_ga=GA1.2.372856789.1607119103; _gid=GA1.2.286951743.1607119103; session=53616c7465645f5ffa9e202d52a368e9535a5ce15a2bee701a6f379397bdabf079f75d0f17f39927e406ef1d3292b8f1; _gat=1".parse().unwrap());
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
            name: v["name"].to_string(),
            coins: v["local_score"].to_string(),
            stars: v["stars"].to_string()
        });
    }
    user_info

}
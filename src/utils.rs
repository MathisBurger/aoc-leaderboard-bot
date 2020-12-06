use crate::api::call_devschuppen_leaderboard::user;
use std::collections::HashMap;
use serenity::model::channel::EmbedField;

pub async fn get_leaderboard(user: Vec<user>) -> String {
    let mut embed_string = String::new();
    for i in 0..user.len() {
        embed_string = format!("{}\n{}) - `{}` - `{}`", embed_string, i + 1, user[i].name.replace("\"", ""), user[i].stars);
    }
    embed_string
}

pub async fn sort_user_by_points(user: Vec<user>) -> Vec<user> {
    let mut sorted_user: Vec<user> = vec![];
    let mut map: HashMap<i32, String> = HashMap::new();
    let mut coin_array: Vec<i32> = vec![];
    for el in user {
        let str = format!("{};{}", el.name, el.stars);
        map.insert(el.coins.parse::<i32>().unwrap(), str);
    }
    for val in map.iter() {
        coin_array.push(*val.0);
    }
    coin_array.sort();
    for placement in coin_array {
        let value = map.get(&placement).unwrap();
        let name_stars = value.split(";").collect::<Vec<&str>>();
        sorted_user.push(user {
            name: name_stars[0].to_string(),
            coins: placement.to_string(),
            stars: name_stars[1].to_string()
        })
    }
    sorted_user
}


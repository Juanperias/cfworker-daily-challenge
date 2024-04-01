use reqwest::Client;
use serde_json::json;
use worker::{console_debug, console_warn};

use crate::challenge::DailyChallenge;

pub async fn set_daily(endpoint: String, day: i64, challenge: DailyChallenge, client: &Client) {
    let req = json!({
        "title": format!("Reto #{day} - {}", challenge.question.title),
        "message": challenge.to_string(),
        "tag_name": challenge.question.difficulty.to_string(),
    });

    let res = client
        .post(endpoint)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))
        .unwrap()
        .text()
        .await
        .inspect_err(|e| console_warn!("Json Error: {e:?}"))
        .unwrap();

    console_debug!("Result: {res:?}");
}

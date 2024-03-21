use reqwest::Client;
use serde_json::json;
use worker::console_warn;

use crate::challenge::{DailyChallenge, GraphQLResponse};

const LETCODE_URL: &str = "https://leetcode.com";

const DAILY_QUERY: &str = r#"query {
    activeDailyCodingChallengeQuestion {
        date
        link
        question {
            title
            content
            isPaidOnly
            difficulty
            exampleTestcases
            topicTags {
                name
                slug
                translatedName
            }
            codeSnippets {
                lang
                langSlug
                code
            }
            hints
            sampleTestCase
        }
    }
}"#;

// Strategy to fetch from https://github.com/JacobLinCool/LeetCode-Query
pub async fn get_daily(client: &Client) -> DailyChallenge {
    let req = json!({
        "oprationName": "globalData",
        "variables": {},
        "query": DAILY_QUERY
    });
    client
        .post(format!("{LETCODE_URL}/graphql"))
        .header("content-type", "application/json")
        .header("origin", LETCODE_URL)
        .header("referer", LETCODE_URL)
        .header("x-csrftoken", "")
        .header("Cookie", "csrftoken=\"\";LEETCODE_SESSION=\"\"")
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))
        .unwrap()
        .json::<GraphQLResponse>()
        .await
        .inspect_err(|e| console_warn!("Json Error: {e:?}"))
        .unwrap()
        .data
        .active_daily_coding_challenge_question
}

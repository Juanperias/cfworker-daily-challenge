use reqwest::Client;
use serde_json::json;

use crate::challenge::DailyChallenge;

const LETCODE_URL: &str = "https://leetcode.com";

const DAILY_QUERY: &str = r#"query {
    activeDailyCodingChallengeQuestion {
        date
        link
        question {
            questionId
            questionFrontendId
            title
            titleSlug
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
            stats
            hints
            sampleTestCase
            metaData
            envInfo
        }
    }
}"#;

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
        .unwrap()
        .json::<DailyChallenge>()
        .await
        .unwrap()
}

use reqwest::Client;

use crate::challenge::DailyChallenge;

const LETCODE_URL: &str = "https://leetcode.com/graphql";

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
            code_snippets {
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
    client
        .post(LETCODE_URL)
        .body(DAILY_QUERY)
        .send()
        .await
        .unwrap()
        .json::<DailyChallenge>()
        .await
        .unwrap()
}

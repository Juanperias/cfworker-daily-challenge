use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLResponse {
    pub data: ActiveDailyCodingChallengeQuestion,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDailyCodingChallengeQuestion {
    pub active_daily_coding_challenge_question: DailyChallenge,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyChallenge {
    pub date: String,
    pub link: String,
    pub question: Problem,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ProblemDifficulty {
    Easy,
    Medium,
    Hard
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicTag {
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub code: String,
    pub lang: String,
    pub lang_slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub title: String,
    pub content: String,
    pub is_paid_only: bool,
    pub difficulty: ProblemDifficulty,
    pub example_testcases: String,
    pub topic_tags: Vec<TopicTag>,
    pub code_snippets: Vec<CodeSnippet>,
    pub hints: Vec<String>,
    pub sample_test_case: String,
}

impl ToString for ProblemDifficulty {
    fn to_string(&self) -> String {
        match self {
            ProblemDifficulty::Easy => "facil".to_owned(),
            ProblemDifficulty::Medium => "intermedio".to_owned(),
            ProblemDifficulty::Hard => "dificil".to_owned(),
        }
    }
}

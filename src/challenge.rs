use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyChallenge {
    date: String,
    link: String,
    question: Problem,
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
    name: String,
    slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    code: String,
    lang: String,
    lang_slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    question_id: String,
    question_frontend_id: String,
    title: String,
    title_slug: String,
    content: String,
    is_paid_only: bool,
    difficulty: ProblemDifficulty,
    example_testcases: String,
    topic_tags: Vec<TopicTag>,
    code_snippets: Vec<CodeSnippet>,
    stats: String,
    hints: Vec<String>,
    sample_test_case: String,
    meta_data: String,
    env_info: String,
}

use std::collections::HashMap;

use html2md::common::get_tag_attr;
use html2md::{parse_html_custom, TagHandler, TagHandlerFactory};
use serde::{Deserialize, Serialize};
use worker::console_debug;

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
    Hard,
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

impl ToString for DailyChallenge {
    fn to_string(&self) -> String {
        console_debug!("Raw html content: {}", self.question.content);
        let mut custom_parser: HashMap<String, Box<dyn TagHandlerFactory>> = HashMap::new();
        custom_parser
            .entry("img".to_owned())
            .or_insert(Box::new(ImgHandler));

        let parsed = parse_html_custom(
            &self
                .question
                .content
                .split('\n')
                .take(30)
                .collect::<String>(),
            &custom_parser,
        );
        console_debug!("Content markdown Parsed: {parsed}");
        let test_cases = format!(
            "- {}",
            self.question.example_testcases.replace('\n', "\n> - ")
        );

        let code = self
            .question
            .code_snippets
            .iter()
            .find(|c| c.lang_slug == "rust")
            .map(|c| format!("```rs\nstruct Solution;\n\n{}\n```", c.code))
            .unwrap_or_default();

        format!(
            r#"{parsed}

> Enlace:
> https://leetcode.com/{}

> Casos de Prueba
> {test_cases}

{code}
"#,
            self.link
        )
    }
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

struct ImgHandler;
impl TagHandler for ImgHandler {
    fn handle(&mut self, tag: &html2md::Handle, printer: &mut html2md::StructuredPrinter) {
        let src = get_tag_attr(tag, "src").unwrap_or_default();
        let alt = get_tag_attr(tag, "alt");

        console_debug!("SRC image content: {src}");
        console_debug!("ALT image content: {alt:?}");

        if let Some(alt) = alt {
            printer.append_str(&format!("![{}]({})", alt, &src));
        } else {
            printer.append_str(&src);
        }
    }

    fn after_handle(&mut self, _printer: &mut html2md::StructuredPrinter) {}
}

impl TagHandlerFactory for ImgHandler {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(ImgHandler)
    }
}

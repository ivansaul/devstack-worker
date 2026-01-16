use crate::cheatsheets::helpers::vec_or_json;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct CheatsheetMetaRow {
    pub id: String,
    pub title: String,
    #[serde(default, deserialize_with = "vec_or_json")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "vec_or_json")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheatsheetRow {
    pub id: String,
    pub title: String,
    #[serde(default, deserialize_with = "vec_or_json")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "vec_or_json")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(default, deserialize_with = "vec_or_json")]
    pub sections: Vec<SectionRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionRow {
    pub title: String,
    pub content: String,
}

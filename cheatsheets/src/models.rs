use crate::helpers::as_hex_color;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, serde_as};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cheatsheet {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub intro: Option<String>,
    pub label: Option<String>,
    pub icon: Option<String>,
    pub background: Option<String>,
    pub sections: Vec<Section>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub(crate) struct CheatsheetMeta {
    pub title: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub tags: Vec<String>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub categories: Vec<String>,
    pub intro: Option<String>,
    pub label: Option<String>,
    #[serde(deserialize_with = "as_hex_color")]
    pub background: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct GithubFile {
    pub name: String,
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Deserialize)]
pub struct CheatsheetSeed {
    pub id: String,
    #[serde(default)]
    pub enabled: bool,
}

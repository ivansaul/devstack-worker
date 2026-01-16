pub mod error;
pub(crate) mod helpers;
pub mod models;

use crate::{
    error::{Error, Result},
    models::{Cheatsheet, CheatsheetMeta, GithubFile, Section},
};
use cached::proc_macro::cached;
use gray_matter::{Matter, engine::YAML};
use markdown::{
    ParseOptions,
    mdast::{Heading, Node},
};
use mdast_util_to_markdown::to_markdown;
use reqwest::Client;
use serde::de::IntoDeserializer;
use serde_json::Value;
use std::collections::HashMap;

pub async fn parse_markdown(id: &str) -> Result<Cheatsheet> {
    let markdown = fetch_markdown(id).await?;
    let frontmatter = parse_frontmatter::<CheatsheetMeta>(&markdown)?;
    let sections = parse_sections(&markdown)?;
    let icon = get_icon_url(id).await?;
    let cheatsheet = Cheatsheet {
        id: id.to_string(),
        title: frontmatter.title,
        intro: frontmatter.intro,
        tags: frontmatter.tags,
        categories: frontmatter.categories,
        label: frontmatter.label,
        icon: Some(icon),
        background: frontmatter.background,
        sections,
    };
    Ok(cheatsheet)
}

fn parse_frontmatter<T>(markdown: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let data = Matter::<YAML>::new()
        .parse::<Value>(markdown)
        .map_err(|e| Error::ParseError(e.to_string()))?
        .data
        .ok_or(Error::ParseError("Missing frontmatter".into()))?;

    let deserializer = data.into_deserializer();
    let fmt = serde_path_to_error::deserialize(deserializer)
        .map_err(|e| Error::ParseError(format!("Frontmatter error: {}", e)))?;

    Ok(fmt)
}

fn parse_sections(markdown: &str) -> Result<Vec<Section>> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current_section_title: Option<String> = None;
    let mut current_section_content: String = String::new();

    let tree = markdown::to_mdast(markdown, &ParseOptions::default())
        .map_err(|message| Error::ParseError(message.reason))?;

    if let Node::Root(root) = tree {
        for child in root.children.iter() {
            match child {
                Node::Heading(Heading { depth: 2, .. }) => {
                    if let Some(title) = current_section_title.take() {
                        let section = Section {
                            title,
                            content: current_section_content.clone(),
                        };
                        sections.push(section);
                        current_section_content.clear();
                    }
                    current_section_title = Some(child.to_string());
                }
                _ => {
                    if current_section_title.is_some() {
                        current_section_content
                            .push_str(to_markdown(child).unwrap_or_default().as_ref());
                    }
                }
            }
        }

        if let Some(title) = current_section_title.take() {
            let section = Section {
                title,
                content: current_section_content,
            };
            sections.push(section);
        }
    }

    Ok(sections)
}

async fn get_icon_url(id: &str) -> Result<String> {
    let cache = load_icon_cache().await?;
    let icon = cache.get(id).cloned().unwrap_or(DEFAULT_ICON_URL.into());
    Ok(icon)
}

#[cached(result = true)]
async fn load_icon_cache() -> Result<HashMap<String, String>> {
    let url = GH_API_ICON_DIR;
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let response = client.get(url).send().await?.error_for_status()?;
    let files: Vec<GithubFile> = response.json().await?;
    let map = files
        .into_iter()
        .filter(|f| f.file_type == "file")
        .filter_map(|f| f.download_url.map(|url| (f.name.replace(".svg", ""), url)))
        .collect();
    Ok(map)
}

async fn fetch_markdown(id: &str) -> Result<String> {
    let url = format!("{}/{}.md", GH_RAW_POSTS_BASE, id);
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let response = client.get(&url).send().await?.error_for_status()?;
    let content = response.text().await?;
    let content = clean_markdown(&content)?;
    Ok(content)
}

fn clean_markdown(markdown: &str) -> Result<String> {
    let mut cleaned = markdown.to_string();
    for (pattern, replacement) in REPLACEMENT_RULES.iter() {
        let re = regex::Regex::new(pattern)
            .map_err(|e| Error::ParseError(format!("Error parsing regex: {}", e)))?;
        cleaned = re.replace_all(&cleaned, *replacement).to_string();
    }
    Ok(cleaned)
}

const GH_API_ICON_DIR: &str =
    "https://api.github.com/repos/Fechin/reference/contents/source/assets/icon";

const GH_RAW_POSTS_BASE: &str =
    "https://raw.githubusercontent.com/Fechin/reference/main/source/_posts/";

const USER_AGENT: &str = "reference-worker/1.0 (+https://github.com/Fechin/reference)";

const DEFAULT_ICON_URL: &str =
    "https://raw.githubusercontent.com/Fechin/reference/main/source/assets/icon/todoist.svg";

type Rule = (&'static str, &'static str);

const REPLACEMENT_RULES: &[Rule] = &[
    // Remove markdown classes / attributes
    (CSS_PATTERN, ""),
    // HTML → Markdown
    (r"(?s)<code>(.*?)</code>", "`$1`"),
    (r"(?s)<yel>(.*?)</yel>", "$1"),
];

const CSS_PATTERN: &str = r#"(?x)
    \{
        \s*
        (?:
            \.(?:cols|rows|col-span|row-span)-\d+
          | \.(?:primary|secondary|wrap|shortcuts|bold-first|plus-first|left-text|no-wrap|show-header|headers|link-arrow)
          | \.(?:marker-(?:none|round|timeline))
          | \.(?:style-[^\s}]+)
        )
        (?:
            \s+
            (?:
                \.(?:cols|rows|col-span|row-span)-\d+
              | \.(?:primary|secondary|wrap|shortcuts|bold-first|plus-first|left-text|no-wrap|show-header|headers|link-arrow)
              | \.(?:marker-(?:none|round|timeline))
              | \.(?:style-[^\s}]+)
            )
        )*
        \s*
    \}
    |
    <!--\s*(?:prettier-ignore|rehype:[^>]+)\s*-->
    |
    [Ss]ee:\s*\[[^\]]+\]\(\#[^)]+\)
    |
    data-tooltip=
    "#;

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("rust")]
    #[case("python")]
    #[case("swift")]
    #[tokio::test]
    async fn test_parse_frontmatter_section(#[case] id: &str) -> anyhow::Result<()> {
        let markdown = fetch_markdown(id).await?;
        let frontmatter = parse_frontmatter::<CheatsheetMeta>(&markdown)?;
        let sections = parse_sections(&markdown)?;
        assert!(!frontmatter.title.is_empty());
        assert!(!frontmatter.categories.is_empty());
        assert!(frontmatter.background.is_some());
        assert!(!sections.is_empty());
        Ok(())
    }

    #[rstest::rstest]
    #[case("rust")]
    #[case("python")]
    #[case("swift")]
    #[tokio::test]
    async fn test_get_icon_url(#[case] id: &str) -> anyhow::Result<()> {
        let icon_url = get_icon_url(id).await?;
        assert!(icon_url.contains(id));
        Ok(())
    }

    #[rstest::rstest]
    #[case("{.cols-1}", "")]
    #[case("{.marker-timeline}", "")]
    #[case("{.row-span-3}", "")]
    #[case("{.style-bold}", "")]
    #[case("{.row-span-1 .col-span-2}", "")]
    #[case("see: [Example](#example)", "")]
    #[case("<!-- prettier-ignore -->", "")]
    #[case("<!-- comment -->", "<!-- comment -->")]
    #[case("<code>rust</code> code", "`rust` code")]
    #[test]
    fn test_clean_markdown(#[case] input: &str, #[case] expected: &str) -> anyhow::Result<()> {
        let cleaned = clean_markdown(input)?;
        assert_eq!(cleaned, expected);
        Ok(())
    }
}

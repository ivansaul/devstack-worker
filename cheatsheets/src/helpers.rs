use serde::{Deserialize, Deserializer};

/// Deserialize a string as a hexadecimal color code.
/// This is useful when deserializing values that may contain additional
/// characters (e.g. CSS utility classes like `bg-[#436b97]`).
pub fn as_hex_color<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let re = regex::Regex::new(r"#([0-9a-fA-F]{6})").map_err(serde::de::Error::custom)?;
    match re.captures(&value) {
        Some(cap) => Ok(Some(cap[0].to_string())),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[rstest::rstest]
    #[case("bg-[#436b97]", Some("#436b97"))]
    #[case("#000000", Some("#000000"))]
    #[case("bg-black", None)]
    fn test_as_hex_color(
        #[case] input: &str,
        #[case] expected: Option<&str>,
    ) -> anyhow::Result<()> {
        let json = json!(input);
        let result = as_hex_color(json)?;
        assert_eq!(result, expected.map(|s| s.to_string()));
        Ok(())
    }
}

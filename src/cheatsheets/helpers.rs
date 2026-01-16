use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::Value;

pub fn vec_or_json<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Null => Ok(Vec::new()),

        Value::String(s) => {
            serde_json::from_str::<Vec<T>>(&s).map_err(|e| serde::de::Error::custom(e.to_string()))
        }

        Value::Array(arr) => arr
            .into_iter()
            .map(|v| serde_json::from_value(v).map_err(|e| serde::de::Error::custom(e.to_string())))
            .collect(),

        _ => Err(serde::de::Error::custom(
            "Expected null, stringified JSON array, or array",
        )),
    }
}

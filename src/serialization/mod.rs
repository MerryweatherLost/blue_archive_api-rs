use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_html_encoded_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let buffer = String::deserialize(deserializer)?;
    Ok(html_escape::decode_html_entities(&buffer).into())
}

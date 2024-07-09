use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_nan_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    if let Some(float) = value.as_f64() {
        return Ok(float);
    }

    if let Some(string) = value.as_str() {
        if string == "NaN" {
            return Ok(f64::NAN);
        }
    }

    Err(de::Error::custom(format!(
        "'{value}' isn't a float or 'NaN' value"
    )))
}

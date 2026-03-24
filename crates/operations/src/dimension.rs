use nonempty::NonEmpty;
use nutype::nutype;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};

type JsonObject = Map<String, Value>;

struct NewDimension;
struct DimensionSettings;

#[nutype(validate(not_empty), derive(Debug, Display, Serialize, Deserialize))]
struct CohortVariant(String);

#[derive(Debug, Serialize, Deserialize)]
struct CohortDefinition {
    variant: CohortVariant,
    /// json-logic expression.
    expr: JsonObject,
}

#[derive(Debug, Serialize)]
struct LocalCohort {
    parent_dim: String,
    r#enum: NonEmpty<CohortVariant>,
    defs: NonEmpty<CohortDefinition>,
    raw: JsonObject,
}

fn get_field<'de, T: DeserializeOwned>(
    map: &Map<String, Value>,
    key: &'static str,
) -> Result<T, String> {
    let val = map
        .get(key)
        .ok_or_else(|| {
            <serde_json::Error as serde::de::Error>::missing_field(key).to_string()
        })?
        .clone();
    serde_json::from_value(val).map_err(|e| e.to_string())
}

impl LocalCohort {
    fn new_unchecked(parent_dim: String, raw: JsonObject) -> Result<Self, String> {
        #[derive(Deserialize)]
        struct Parsed {
            definitions: JsonObject,
            r#enum: NonEmpty<CohortVariant>
        }
        let Parsed { definitions, r#enum } = serde_json::from_value(Value::Object(raw)).map_err(|e| e.to_string())?;
        if definitions.is_empty() {
            return Err("Need at least one definition.");
        }
    }
}

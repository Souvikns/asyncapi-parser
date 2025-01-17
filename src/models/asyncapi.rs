use crate::models::info::Info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AsyncAPIDocument {
    asyncapi: String,
    info: Info,
}

impl AsyncAPIDocument {
    pub fn version(&self) -> &str {
        &self.asyncapi
    }

    pub fn info(&self) -> &Info {
        &self.info
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_spec_file() {
        let json_spec = json!({
            "asyncapi": "3.0.0",
            "info": {
                "title": "Simple web server",
                "version": "0.0.1"
            }
        });
        let asyncapi: AsyncAPIDocument = serde_json::from_str(&json_spec.to_string()).unwrap();
        assert_eq!(asyncapi.version(), "3.0.0");
        assert_eq!(asyncapi.info().version(), "0.0.1");
        assert_eq!(asyncapi.info().title(), "Simple web server");
        assert!(!asyncapi.info().has_description());
        assert!(!asyncapi.info().has_contact());
    }
}

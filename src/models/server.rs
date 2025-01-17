use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    host: String,
    protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    variable: IndexMap<String, ServerVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    en: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<String>>, // TODO: security, tags, externalDocs, bindings
}

impl Server {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn has_path_name(&self) -> bool {
        if self.path_name.is_none() {
            return false;
        }
        true
    }

    pub fn path_name(&self) -> &Option<String> {
        &self.path_name
    }

    pub fn has_protocol_version(&self) -> bool {
        if self.protocol_version.is_none() {
            return false;
        }
        true
    }

    pub fn protocol_version(&self) -> &Option<String> {
        &self.protocol_version
    }

    pub fn has_description(&self) -> bool {
        if self.description.is_none() {
            return false;
        }
        true
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_server() {
        let json_server = json!({
            "host": "127.0.0.1:8080",
            "protocol": "wss"
        });

        let server: Server = serde_json::from_str(&json_server.to_string()).unwrap();
        assert_eq!(server.protocol, "wss");
        assert!(!server.has_description());
    }
}

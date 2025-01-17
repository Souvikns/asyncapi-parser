use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    title: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Contact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Info {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn has_description(&self) -> bool {
        if self.description == None {
            return false;
        }
        true
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn has_id(&self) -> bool {
        if self.id == None {
            return false;
        }
        true
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    pub fn has_terms_of_service(&self) -> bool {
        if self.id == None {
            return false;
        }
        true
    }

    pub fn terms_of_service(&self) -> &Option<String> {
        &self.terms_of_service
    }

    pub fn has_contact(&self) -> bool {
        if self.contact == None {
            return false;
        }

        true
    }

    pub fn contact(&self) -> &Option<Contact> {
        &self.contact
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_info() {
        let json_info = json!({
            "title": "Simple Server".to_string(),
            "version": "0.0.1".to_string()
        });

        let info: Info = serde_json::from_str(&json_info.to_string()).unwrap();
        assert_eq!(info.title(), "Simple Server");
    }
}

use crate::models::info::Info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AsyncAPIDocument {
    asyncapi: String,
    info: Info,
}

impl AsyncAPIDocument {
    pub fn asyncapi(self) -> String {
        self.asyncapi
    }

    pub fn info(self) -> Info {
        self.info
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    tp: Option<SecuritySchemeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in")]
    inn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_id_connect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flows: Option<OAuthFlows>, // TODO: Finish Oauthflows
    #[serde(skip_serializing_if = "Vec::is_empty")]
    scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum SecuritySchemeType {
    #[serde(rename = "userPassword")]
    UserPassword,
    #[serde(rename = "apiKey")]
    ApiKey,
    X509,
    #[serde(rename = "symmetricEncryption")]
    SymmetricEncryption,
    #[serde(rename = "asymmetricEncryption")]
    AsymmetricEncryption,
    #[serde(rename = "httpApiKey")]
    HttpApiKey,
    #[serde(rename = "http", rename_all = "camelCase")]
    Http,
    #[serde(rename = "oAuth2")]
    OAuth2,
    #[serde(rename = "openIdConnect", rename_all = "camelCase")]
    OpenIdConnect,
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "scramSha256")]
    ScramSha256,
    #[serde(rename = "scramSha512")]
    ScramSha512,
    #[serde(rename = "gssapi")]
    Gssapi,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {}

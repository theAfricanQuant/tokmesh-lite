use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub owner: Owner,
    pub data: DataSpec,
    #[serde(default)]
    pub quality: Vec<QualityRule>,
    pub sovereignty: Sovereignty,
    #[serde(default)]
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub country: String,
    pub organization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSpec {
    pub format: DataFormat,
    pub schema: Vec<Column>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataFormat {
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: DataType,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Integer,
    Decimal,
    Boolean,
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRule {
    pub column: String,
    pub rule: RuleType,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Unique,
    Range,
    #[serde(rename = "accepted_values")]
    AcceptedValues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sovereignty {
    pub classification: String,
    #[serde(default)]
    pub allowed_countries: Vec<String>,
}

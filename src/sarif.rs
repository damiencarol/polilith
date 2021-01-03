use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SarifLog {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<Run>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Run {
    pub tool: Tool,
    pub artifacts: Vec<Artifact>,
    pub results: Vec<Result>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Tool {
    pub driver: Driver,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Driver {
    pub name: String,
    pub information_uri: String,
    pub full_name: String,
    pub version: String,
    pub rules: Vec<ReportingDescriptor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Artifact {
    pub location: ArtifactLocation,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ArtifactLocation {
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Result {
    pub rule_id: String,
    pub kind: String,
    pub level: String,
    pub message: ResultMessage,
    pub locations: Vec<ResultLocation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResultLocation {
    pub physical_location: PhysicalLocation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PhysicalLocation {
    pub artifact_location: ArtifactLocation,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResultMessage {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: std::option::Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReportingDescriptor {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: std::option::Option<MultiformatMessageString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: std::option::Option<MultiformatMessageString>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MultiformatMessageString {
    pub text: String,
}

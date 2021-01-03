use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tar::Archive;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerManifest {
    pub config: String,
    pub repo_tags: Vec<String>,
    pub layers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DockerConfig {
    pub config: DockerConfigConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerConfigConfig {
    pub user: String,
    pub env: Vec<String>,
}

pub(crate) fn get_manifest(ar: &mut Archive<File>) -> Vec<DockerManifest> {
    let mut manifest_data = String::new();
    for file in ar.entries().unwrap() {
        let mut file = file.unwrap();
        if Path::new("manifest.json") == file.header().path().unwrap() {
            file.read_to_string(&mut manifest_data).unwrap();
        }
    }
    let manifest: Vec<DockerManifest> = serde_json::from_str(&manifest_data).unwrap();
    manifest
}

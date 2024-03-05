use serde::Deserialize;

use super::MojangApiWrapper;


#[derive(Debug, Deserialize)]
pub(crate) struct VersionJson {
  #[serde(rename = "assetIndex")]
  pub asset_index: AssetIndex,
  assets: String,
  downloads: Downloads,
  id: String,
  #[serde(rename = "javaVersion")]
  java_version: JavaVersion,
  libraries: Vec<Library>,
  #[serde(rename = "mainClass")]
  main_class: String,  
}

#[derive(Debug, Deserialize)]
struct Rule {
  action: String,
  os: Option<Os>,
}

#[derive(Debug, Deserialize)]
struct Os {
  name: String,
  version: Option<String>,
  arch: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AssetIndex {
  pub id: String,
  pub sha1: String,
  pub size: u64,
  #[serde(rename = "totalSize")]
  pub total_size: u64,
  pub url: String,
}

#[derive(Debug, Deserialize)]
struct Downloads {
  client: Client,
  // we don't need server for now
}

#[derive(Debug, Deserialize)]
struct Client {
  sha1: String,
  size: u64,
  url: String,
}

#[derive(Debug, Deserialize)]
struct JavaVersion {
  component: String,
  #[serde(rename = "majorVersion")]
  major: u64,
}

#[derive(Debug, Deserialize)]
struct Library {
  downloads: Option<LibraryDownloads>,
  name: String,
  natives: Option<Natives>,
  rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize)]
struct LibraryDownloads {
  artifact: Artifact,
  classifiers: Option<Classifiers>,
}

#[derive(Debug, Deserialize)]
struct Artifact {
  path: String,
  sha1: String,
  size: u64,
  url: String,
}

#[derive(Debug, Deserialize)]
struct Classifiers {
  #[serde(rename = "natives-linux")]
  natives_linux: Option<Native>,
  #[serde(rename = "natives-osx")]
  natives_osx: Option<Native>,
  #[serde(rename = "natives-windows")]
  natives_windows: Option<Native>,
}

#[derive(Debug, Deserialize)]
struct Native {
  path: String,
  sha1: String,
  size: u64,
  url: String,
}

#[derive(Debug, Deserialize)]
struct Natives {
  linux: Option<String>,
  osx: Option<String>,
  windows: Option<String>,
}


#[derive(Debug, Deserialize)]
struct File {
  id: String,
  sha1: String,
  size: u64,
  url: String,
}

impl MojangApiWrapper {
  pub async fn get_version_json(&self, version: impl ToString) -> Result<VersionJson, reqwest::Error> {
    let version_manifest = self.get_version_manifest().await?;
    // get the sha1 of the version
    let version_manifest_filtered = version_manifest.versions.iter().find(|v| v.id == version.to_string()).unwrap();
    let url = format!("https://launchermeta.mojang.com/v1/packages/{}/{}.json", version_manifest_filtered.sha1, version.to_string());
    let response = self.http_client.get(&url).send().await?.json().await?;
    Ok(response)
  }
}

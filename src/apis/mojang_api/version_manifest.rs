use serde::Deserialize;

use super::MojangApiWrapper;

const url: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Deserialize)]
pub(crate) struct VersionManifestV2 {
    latest: Latest,
    pub(crate) versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Version {
    pub (crate) id: String,
    r#type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    pub(crate) sha1: String,
}



impl MojangApiWrapper {
    pub(crate) async fn get_version_manifest(&self) -> Result<VersionManifestV2, reqwest::Error> {
        let response = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }
}
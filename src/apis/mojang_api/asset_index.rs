use std::collections::HashMap;

use serde::Deserialize;

use super::MojangApiWrapper;
use super::version_json::AssetIndex;


#[derive(Deserialize, Debug)]
pub struct RemoteAssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

impl MojangApiWrapper {
    pub async fn get_asset_index(&self, index: AssetIndex ) -> Result<RemoteAssetIndex, Box<dyn std::error::Error>> {
        let response = self.http_client.get(index.url).send().await?.json().await?;
        Ok(response)
    }
}
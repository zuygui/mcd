pub(crate) mod version_manifest;
pub(crate) mod version_json;

pub(crate) struct MojangApiWrapper {
  http_client: reqwest::Client,
}

impl MojangApiWrapper {
  pub fn new() -> Self {
    Self {
      http_client: reqwest::Client::new(),
    }
  }
}
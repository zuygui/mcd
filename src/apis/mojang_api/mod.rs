mod version_manifest;
mod version_json;

struct MojangApiWrapper {
  http_client: reqwest::Client,
}
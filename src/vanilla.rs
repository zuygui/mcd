use crate::apis::mojang_api::{version_json::VersionJson, MojangApiWrapper};

pub struct VanillaVersion {
  // TODO: version_json (resources, assets, libraries, etc)
  version_json: VersionJson,
}

pub struct VanillaVersionBuilder {  
  version_json: Option<VersionJson>,
}

impl VanillaVersionBuilder {

  pub fn new() -> Self {
    Self {
      version_json: None,
    }
  }

  pub async fn with_version_id(&mut self, version_id: impl ToString) -> Result<(), Box<dyn std::error::Error>> { 
    
    self.version_json = Some(MojangApiWrapper::new().get_version_json(version_id).await?);

    Ok(())
  }
  
  pub fn build(self) -> VanillaVersion {
    VanillaVersion {
      version_json: self.version_json.unwrap(),
    }
  }

}
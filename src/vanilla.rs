use crate::apis::mojang_api::{version_json::VersionJson, MojangApiWrapper};

pub struct VanillaVersion {
  /// resources, assets, libraries, etc
  version_json: VersionJson,
}

impl VanillaVersion {
  pub(crate) fn update_assets(&self) {
    unimplemented!()
  }

  pub(crate) fn update_resources(&self) {
    unimplemented!()
  }

  pub(crate) fn update_libraries(&self) {
    unimplemented!()
  }

  pub(crate) fn update_natives(&self) {
    unimplemented!()
  }

  pub(crate) fn update_client(&self) {
    unimplemented!()
  }

  pub(crate) fn update(&self) {
    unimplemented!()
  }
}

pub struct VanillaVersionBuilder {  
  version_id: Option<String>,
}

impl VanillaVersionBuilder {

  pub fn new() -> Self {
    Self {
      version_id: None,
    }
  }

  pub fn with_version_id(&mut self, version_id: impl ToString) -> &mut Self { 
    
    self.version_id = Some(version_id.to_string());
    self

  }
  
  pub async fn build(&self) -> Result<VanillaVersion, Box<dyn std::error::Error>> {

    if self.version_id.is_none() {
      return Err("version_id is required".into());
    }

    let version_json = MojangApiWrapper::new().get_version_json(<std::option::Option<std::string::String> as Clone>::clone(&self.version_id).unwrap()).await?;

    Ok(VanillaVersion {
      version_json
    })
  }

}
use std::path::PathBuf;

use futures::{FutureExt, StreamExt};
use tokio::task::{self, JoinHandle};

use crate::apis::mojang_api::{version_json::VersionJson, MojangApiWrapper};

pub struct VanillaVersion {
  /// resources, assets, libraries, etc
  version_json: VersionJson,
}

impl VanillaVersion {
  pub(crate) async fn update_assets(&self, game_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Get asset index
    let asset_index = MojangApiWrapper::new().get_asset_index(self.version_json.asset_index.clone()).await?;

    // Download with SHA-1 verification and retry logic
    let mut tasks: Vec<JoinHandle<Result<(), Box<std::io::Error>>>> = Vec::new();
    for (key, value) in asset_index.objects {
        let game_dir = game_dir.clone();
        let key = key.clone();
        let value = value.clone();

        let task = task::spawn(async move {
            const MAX_RETRIES: usize = 3;
            let mut retries = 0;

            while retries < MAX_RETRIES {
                match reqwest::get(format!("https://resources.download.minecraft.net/{}/{}", value.hash[..2].to_string(), value.hash)).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let asset = response.bytes().await.unwrap();
                            let asset_path = game_dir.join("assets").join(key.clone());
                            std::fs::create_dir_all(asset_path.parent().unwrap()).unwrap();
                            tokio::fs::write(asset_path, asset).await?;
                            return Ok(());
                        }
                    }
                    Err(_) => {
                        retries += 1;
                    }
                }
            }

            // If all retries fail, return an error
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download asset")))
        });

        tasks.push(task);
    }

    // Wait for all tasks to finish
    for task in tasks {
        task.await?;
    }

    Ok(())
}
  

  pub(crate) async fn update_libraries(&self, game_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("TODO: update_libraries");
    Ok(())
  }

  pub(crate) async fn update_client(&self, game_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("TODO: update_client");
    Ok(())
  }

  pub(crate) async fn update(&self, game_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    std::fs::create_dir_all(game_dir.clone()).unwrap();
    std::fs::create_dir_all(game_dir.join("assets")).unwrap();
    std::fs::create_dir_all(game_dir.join("libraries")).unwrap();
    std::fs::create_dir_all(game_dir.join("versions")).unwrap();


    self.update_assets(game_dir.clone()).await?;
    self.update_libraries(game_dir.clone()).await?;
    self.update_client(game_dir).await?;

    Ok(())
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
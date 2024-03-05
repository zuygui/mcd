use std::path::Path;

use mcd::{updater::UpdaterBuilder, vanilla::VanillaVersionBuilder};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let vanilla_version = VanillaVersionBuilder::new()
    .with_version_id("1.16.5")
    .build()
    .await?;

  let updater = UpdaterBuilder::new()
    .with_vanilla_version(vanilla_version)
    .with_game_dir(Path::new(".minecraft"))
    .build();

  updater.update().await?;

  Ok(())
}

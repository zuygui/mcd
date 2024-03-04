
/*
The Updater will be organized in the following way:

- One Vanilla Version (Basic files of Minecraft)

- Tweak (Forge, Fabric, Magma, etc) 

- Extra Files (Ressource Packs, etc. NO MODS)
*/

use crate::vanilla::VanillaVersion;

/// The Updater
/// 
/// An Updater is a struct that contains all the necessary files to update the game and install required modules (Modloaders, extra files)
pub struct Updater {
    /// A Vanilla Version Instance
    /// 
    /// This is the basic files of Minecraft (resources, assets, libraries, etc)
    /// 
    /// This is the first step to update the game
    vanilla: VanillaVersion,

    // tweak ?

    // extra_files ?
}

impl Updater {
    /// Update the Game
    pub fn update(&self) {
        unimplemented!()
    }
}

/// The Updater Builder
/// 
/// An Updater Builder is a struct that allows to create an Updater modularly
pub struct UpdaterBuilder {
    vanilla: Option<VanillaVersion>,
}

impl UpdaterBuilder {
    /// Create a new Updater Builder
    pub fn new() -> Self {
        Self {
            vanilla: None,
        }
    }

    /// Set the Vanilla Version
    pub fn with_vanilla_version(mut self, vanilla: VanillaVersion) -> Self {
        self.vanilla = Some(vanilla);
        self
    }

    /// Build the Updater
    pub fn build(self) -> Updater {
        Updater {
            vanilla: self.vanilla.unwrap(),
        }
    }
}
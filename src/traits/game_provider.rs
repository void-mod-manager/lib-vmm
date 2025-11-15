use std::path::PathBuf;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::registry::model::ProviderSource;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum GameIcon {
    Path(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GameInstallError {
    #[error("Mod archive is invalid or corrupted")]
    InvalidArchive,
    #[error("Required game files are missing, is it installed?")]
    MissingGameFiles,
    #[error("Filesystem error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Provider error: {message}")]
    Other{
        /// This message is shown on the frontend, maybe :)
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GameMetadata {
    pub id: String,
    pub display_name: String,
    pub short_name: String,
    pub icon: GameIcon,
    pub provider_source: ProviderSource
}

#[async_trait]
pub trait GameProvider: Send + Sync {
    fn game_id(&self) -> &str;
    fn mod_provider_id(&self) -> &str;
    fn metadata(&self) -> GameMetadata;
    fn get_external_id(&self) -> &str;
    fn install_mod(&self, path: &PathBuf) -> Result<(), GameInstallError>;
}

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::traits::{game_provider::GameProvider, mod_provider::ModProvider};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ProviderSource {
    Core,
    Plugin(String), // pluginId/Name
}

pub struct ProviderEntry {
    pub id: String,
    pub source: ProviderSource,
    pub provider: Arc<dyn ModProvider>,
}

pub struct GameEntry {
    pub id: String,
    pub source: ProviderSource,
    pub game: Arc<dyn GameProvider + Send + Sync>,
    pub required_provider_id: String
}

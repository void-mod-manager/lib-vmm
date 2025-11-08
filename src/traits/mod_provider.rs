use std::path::PathBuf;

use async_trait::async_trait;

use crate::traits::discovery::{DiscoveryError, DiscoveryQuery, DiscoveryResult, ModExtendedMetadata, ModSummary};


/// Note: Currently unimplemented
#[derive(Default, Debug)]
pub struct ModProviderFeatures {
    pub supports_endorsements: bool,
    pub requires_api_token: bool,
    pub mod_multi_file: bool
}

pub enum ModDownloadResult {
    Failed(String),
    InProgress(u8),
    Completed(PathBuf),
    Cancelled,
    CannotComplete(String)
}

#[async_trait]
pub trait ModProvider: Send + Sync {
    async fn download_mod(&self, mod_id: String) -> ModDownloadResult;
    async fn discover(&self, query: &DiscoveryQuery) -> Result<DiscoveryResult, DiscoveryError>;

    #[deprecated(since = "0.1.0", note = "Use `discover` instead")]
    #[allow(unused_variables)]
    async fn discover_mods(&self, game_id: String) -> Vec<ModSummary> {
        panic!("Do not use deprecated discover_mod function")
    }

    async fn get_extended_mod(&self, mod_id: &str) -> ModExtendedMetadata;
    fn configure(&self) -> &ModProviderFeatures;

    fn register(&self) -> String {
        todo!("Return provider ID");
    }
}

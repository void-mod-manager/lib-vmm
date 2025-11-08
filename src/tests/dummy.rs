use std::path::PathBuf;

use async_trait::async_trait;

use crate::{registry::model::ProviderSource, traits::{discovery::{DiscoveryError, DiscoveryMeta, DiscoveryQuery, DiscoveryResult, ModExtendedMetadata, ModSummary, PaginationMeta, Tag}, game_provider::{GameIcon, GameMetadata, GameProvider}, mod_provider::{ModDownloadResult, ModProvider, ModProviderFeatures}}};

pub struct DummyModProvider {
    id: String,
    features: ModProviderFeatures,
}

impl DummyModProvider {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            features: ModProviderFeatures::default()
        }
    }
}

#[async_trait]
impl ModProvider for DummyModProvider {
    async fn download_mod(&self, mod_id: String) -> ModDownloadResult {
        if mod_id == "fail" {
            ModDownloadResult::Failed("bad id".into())
        } else {
            ModDownloadResult::Completed(PathBuf::from(format!("/tmp/{}", mod_id)))
        }
    }

    async fn discover(&self, query: &DiscoveryQuery) -> Result<DiscoveryResult, DiscoveryError> {
        let summary = ModSummary {
            id: "mod-1".into(),
            name: "Test Mod".into(),
            description: "Long description".into(),
            short_description: "Short".into(),
            downloads: 42,
            views: 10,
            likes: 5,
            thumbnail_image: "/thumb.png".into(),
            tags: vec!["tag1".into()],
            user_name: "tester".into(),
            user_avatar: "/avatar.png".into(),
        };
        Ok(DiscoveryResult {
            meta: DiscoveryMeta {
                provider_id: self.id.clone(),
                game_id: query.game_id.clone(),
                pagination: PaginationMeta { current: 1, page_size: 10, total_pages: Some(1), total_items: Some(1) },
                applied_tags: query.tags.clone().unwrap_or_default(),
                available_tags: Some(vec![Tag { id: "tag1".into(), name: "Tag One".into() }])
            },
            mods: vec![summary]
        })
    }

    async fn get_extended_mod(&self, mod_id: &str) -> ModExtendedMetadata {
        ModExtendedMetadata {
            header_image: "/header.png".into(),
            carousel_images: vec!["/c1.png".into(), "/c2.png".into()],
            version: "1.0.0".into(),
            installed: mod_id == "installed-mod",
            description: format!("Extended meta for {}", mod_id)
        }
    }

    fn configure(&self) -> &ModProviderFeatures {
        &self.features
    }
}


pub struct DummyGameProvider {
    id: String,
    mod_provider: String
}

impl DummyGameProvider {
    pub fn new(id: &str, mod_provider: &str) -> Self {
        Self { id: id.to_string(), mod_provider: mod_provider.to_string() }
    }
}

#[async_trait]
impl GameProvider for DummyGameProvider {
    fn game_id(&self) -> &str { &self.id }
    fn mod_provider_id(&self) -> &str { &self.mod_provider }
    fn metadata(&self) -> GameMetadata {
        GameMetadata {
            id: self.id.clone(),
            display_name: "Dummy Game".into(),
            short_name: "DG".into(),
            icon: GameIcon::Path("/icon.png".into()),
            provider_source: ProviderSource::Plugin("plugin-x".into())
        }
    }
    fn get_external_id(&self) -> &str { "external-123" }
    fn install_mod(&self, _path: &PathBuf) -> Result<(), ()> { Ok(()) }
}

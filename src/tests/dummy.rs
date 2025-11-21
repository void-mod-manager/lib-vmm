use std::{fmt::Debug, path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::{
    capabilities::{
        api_key_capability::{ApiKeyValidationError, KeyAction, RequiresApiKey}, base::CapabilityRef, builder::CapabilityBuilder, form::{Field, FieldType, FormSchema}
    },
    registry::model::ProviderSource,
    traits::{
        discovery::{
            DiscoveryError, DiscoveryMeta, DiscoveryQuery, DiscoveryResult, ModExtendedMetadata,
            ModSummary, PaginationMeta, Tag,
        },
        game_provider::{GameIcon, GameInstallError, GameMetadata, GameProvider},
        mod_provider::{ModDownloadResult, ModProvider, ModProviderFeatures},
        provider::Provider,
    },
};


pub struct DummyModProvider {
    id: String,
    features: ModProviderFeatures,
    caps: Vec<CapabilityRef>
}

impl Debug for DummyModProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DummyModProvider")
            .field("id", &self.id)
            .field("features", &self.features)
            .finish()
    }
}

impl DummyModProvider {
    pub fn new(id: &str) -> Arc<Self> {
        Arc::new_cyclic(|weak_self| {
            let caps = CapabilityBuilder::new_from_weak(weak_self.clone())
                .api_key()
                .finish();

            DummyModProvider {
                id: id.to_string(),
                features: ModProviderFeatures::default(),
                caps,
            }
        })
    }

    pub fn id_str(&self) -> &str {
        &self.id
    }

    pub fn features(&self) -> &ModProviderFeatures {
        &self.features
    }
}


impl Provider for DummyModProvider {
    fn id(&self) -> &'static str { "a" }
    fn capabilities(&self) -> &[CapabilityRef] { &self.caps }
}

impl RequiresApiKey for DummyModProvider {
    fn on_provided(&self, key: &str) -> Result<KeyAction, ApiKeyValidationError> {
        if key.trim().is_empty() {
            return Err(ApiKeyValidationError::Empty)
        }
        if key.len() < 16 {
            return Err(ApiKeyValidationError::TooShort { min_len: 16 });
        }

        Ok(KeyAction::Store)
    }

    fn needs_prompt(&self, existing_key: Option<&str>) -> bool {
        match existing_key {
            None => true,
            Some(k) if k.is_empty() => true,
            Some(_) => false,
        }
    }

    fn render(&self) -> FormSchema {
        FormSchema { title: "Enter key".into(), description: Some("Description".into()), fields: vec![ Field {
            id: "api_key".into(),
            label: "api_key".into(),
            field_type: Some(FieldType::Password),
            regex: None,
            help: None,
            placeholder: Some("Paste key here".into()),
        }] }
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
                provider_id: self.id_str().to_string(),
                game_id: query.game_id.clone(),
                pagination: PaginationMeta {
                    current: 1,
                    page_size: 10,
                    total_pages: Some(1),
                    total_items: Some(1),
                },
                applied_tags: query.tags.clone().unwrap_or_default(),
                available_tags: Some(vec![Tag {
                    id: "tag1".into(),
                    name: "Tag One".into(),
                }]),
            },
            mods: vec![summary],
        })
    }

    async fn get_extended_mod(&self, mod_id: &str) -> ModExtendedMetadata {
        ModExtendedMetadata {
            header_image: "/header.png".into(),
            carousel_images: vec!["/c1.png".into(), "/c2.png".into()],
            version: "1.0.0".into(),
            installed: mod_id == "installed-mod",
            description: format!("Extended meta for {}", mod_id),
        }
    }

    fn configure(&self) -> &ModProviderFeatures {
        self.features()
    }
}

pub struct DummyGameProvider {
    id: String,
    mod_provider: String,
}

impl DummyGameProvider {
    pub fn new(id: &str, mod_provider: &str) -> Self {
        Self {
            id: id.to_string(),
            mod_provider: mod_provider.to_string(),
        }
    }
}

impl Provider for DummyGameProvider {
    fn id(&self) -> &'static str { "dummy.game" }
    fn capabilities(&self) -> &[CapabilityRef] { &[] }
}

#[async_trait]
impl GameProvider for DummyGameProvider {
    fn mod_provider_id(&self) -> &str {
        &self.mod_provider
    }

    fn game_id(&self) -> &str { &self.id }

    fn metadata(&self) -> GameMetadata {
        GameMetadata {
            id: self.id.clone(),
            display_name: "Dummy Game".into(),
            short_name: "DG".into(),
            icon: GameIcon::Path("/icon.png".into()),
            provider_source: ProviderSource::Plugin("plugin-x".into()),
        }
    }
    fn get_external_id(&self) -> &str {
        "external-123"
    }
    fn install_mod(&self, _path: &PathBuf) -> Result<(), GameInstallError> {
        Ok(())
    }
}

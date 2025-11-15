use serde::{Deserialize, Serialize};


/// The supported sort orders of VMM's discovery page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Relevance,
    Downloads,
    Views,
    Likes,
    Newest,
    Updated,
}

/// The query parameters for VMM's discovery page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQuery {
    /// The ID of the game to filter by
    pub game_id: String,
    /// The target page of results
    pub page: Option<u32>,
    /// The target page size
    pub page_size: Option<u32>,
    /// The target search query
    pub search: Option<String>,
    /// The actively applied filters
    pub tags: Option<Vec<String>>,
    /// The target sort mode
    pub sort: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PaginationMeta {
    pub current: u32,
    pub page_size: u32,
    pub total_pages: Option<u32>,
    pub total_items: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DiscoveryMeta {
    pub provider_id: String,
    pub game_id: String,
    pub pagination: PaginationMeta,
    pub applied_tags: Vec<String>,
    pub available_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DiscoveryResult {
    pub meta: DiscoveryMeta,
    pub mods: Vec<ModSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ModSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub downloads: u32,
    pub views: u32,
    pub likes: u32,
    pub thumbnail_image: String,
    pub tags: Vec<String>,
    pub user_name: String,
    pub user_avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ModExtendedMetadata {
    pub header_image: String,
    pub carousel_images: Vec<String>,
    pub version: String,
    pub installed: bool,
    pub description: String,
}

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum DiscoveryError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    #[error("The required provider is unavailable")]
    ProviderUnavailable,
    #[error("Internal error: {0}")]
    Internal(String),
}

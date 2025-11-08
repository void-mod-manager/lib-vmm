use async_trait::async_trait;
use tokio::sync::watch;

use crate::traits::mod_provider::ModDownloadResult;

#[allow(dead_code, reason="Exposed for library")]
pub struct QueuedDownload {
    pub mod_id: String,
    pub url: String,
}

#[async_trait]
pub trait DownloadService: Send + Sync {
    async fn queue_download(&self, url: String) -> watch::Receiver<ModDownloadResult>;
}

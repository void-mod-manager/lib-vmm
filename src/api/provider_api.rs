use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{watch, OnceCell};

use crate::{services::DownloadService, runtime::context::Context, traits::mod_provider::ModDownloadResult};

/// API for interacting with Void Mod Manager
#[async_trait]
pub trait ProviderApi: Send + Sync {
    fn download_service(&self) -> Arc<dyn DownloadService>;
    fn context(&self) -> Arc<Context>;
    fn set_context(&self, ctx: Arc<Context>);
    async fn queue_download(&self, url: String) -> watch::Receiver<ModDownloadResult>;
}

/// The default implementation of ProviderAPI as used in Void Mod Manager
///
/// This should probably be locked behind `default-services` feature flag, as this isn't required for making plugins
pub struct DefaultProviderApi {
    download_service: Arc<dyn DownloadService>,
    context_cell: OnceCell<Arc<Context>>
}

impl DefaultProviderApi {
    pub fn new(download_service: Arc<dyn DownloadService>) -> Self {
        Self { download_service, context_cell: OnceCell::new() }
    }

    pub fn into_arc(self) -> Arc<dyn ProviderApi> {
        Arc::new(self)
    }
}

#[async_trait]
impl ProviderApi for DefaultProviderApi {
    fn download_service(&self) -> Arc<dyn DownloadService> {
        Arc::clone(&self.download_service)
    }

    fn context(&self) -> Arc<Context> {
        match self.context_cell.get() {
            Some(ctx) => Arc::clone(ctx),
            None => panic!("Context not set!")
        }
    }

    fn set_context(&self, ctx: Arc<Context>) {
        if self.context_cell.set(ctx).is_err() {
            panic!("Cannot set context twice!")
        }
    }

    async fn queue_download(&self, url: String) -> watch::Receiver<ModDownloadResult> {
        self.download_service.queue_download(url).await
    }
}

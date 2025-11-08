use std::sync::Arc;

use crate::{registry::{RegistryError, model::ProviderSource}, runtime::context::ContextBuilder, tests::dummy::{DummyGameProvider, DummyModProvider}};

#[test]
fn register_and_freeze() {
    let mut b = ContextBuilder::new();
    b.register_mod_provider("mod:provider", Arc::new(DummyModProvider::new("mod:provider")), ProviderSource::Plugin("plug-a".into())).unwrap();
    b.register_mod_provider("core:base", Arc::new(DummyModProvider::new("core:base")), ProviderSource::Core).unwrap();

    let gp = Arc::new(DummyGameProvider::new("game-x", "mod:provider"));
    b.register_game_provider(gp, ProviderSource::Plugin("plug-a".into())).unwrap();

    let ctx = b.freeze();
    assert_eq!(ctx.list_mod_providers().len(), 2);
    assert_eq!(ctx.list_games().len(), 1);
}

#[test]
fn reserved_core_id_error() {
    let mut b = ContextBuilder::new();
    let err = b.register_mod_provider("core:evil", Arc::new(DummyModProvider::new("core:evil")), ProviderSource::Plugin("plug".into())).unwrap_err();
    assert!(matches!(err, RegistryError::ReservedCoreId(_)))
}

#[test]
fn missing_dependency_game_registration() {
    let mut b = ContextBuilder::new();
    let gp = Arc::new(DummyGameProvider::new("game-y", "mod:missing"));
    let err = b.register_game_provider(gp, ProviderSource::Plugin("plug".into())).unwrap_err();
    assert!(matches!(err, RegistryError::NotFound(_)));
}

#[test]
fn activation_and_active_provider() {
    let mut b = ContextBuilder::new();
    b.register_mod_provider("mod:p", Arc::new(DummyModProvider::new("mod:p")), ProviderSource::Plugin("p1".into())).unwrap();
    let gp = Arc::new(DummyGameProvider::new("game-z", "mod:p"));
    b.register_game_provider(gp, ProviderSource::Plugin("p1".into())).unwrap();
    let ctx = b.freeze();

    ctx.activate_game("game-z").unwrap();
    assert_eq!(ctx.active_game().unwrap(), "game-z");
    assert_eq!(ctx.active_game_required_provider().unwrap(), "mod:p");
}

// Generic tests

#[tokio::test]
async fn extended_info_error_without_active_game() {
    let mut b = ContextBuilder::new();
    b.register_mod_provider("mod:p", Arc::new(DummyModProvider::new("mod:p")), ProviderSource::Plugin("plug".into())).unwrap();
    let gp = Arc::new(DummyGameProvider::new("game-a", "mod:p"));
    b.register_game_provider(gp, ProviderSource::Plugin("plug".into())).unwrap();
    let ctx = b.freeze();

    let err = ctx.get_extended_info("mod-xyz").await.unwrap_err();

    assert!(matches!(err, RegistryError::NotFound(_))); // No active game
}

#[tokio::test]
async fn extended_info_success() {
    let mut b = ContextBuilder::new();
    b.register_mod_provider("mod:p", Arc::new(DummyModProvider::new("mod:p")), ProviderSource::Plugin("plug".into())).unwrap();
    let gp = Arc::new(DummyGameProvider::new("game-a", "mod:p"));
    b.register_game_provider(gp, ProviderSource::Plugin("plug".into())).unwrap();
    let ctx = b.freeze();
    ctx.activate_game("game-a").unwrap();

    let meta = ctx.get_extended_info("installed-mod").await.unwrap();
    assert!(meta.installed);
}

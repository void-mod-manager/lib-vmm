use std::sync::Arc;

use crate::{capabilities::{api_key_capability::{ApiKeyCapability, ApiKeyValidationError, KeyAction, RequiresApiKey}, base::{Capability, CapabilityCastExt, CapabilityRef}, builder::CapabilityBuilder, ids}, capability, tests::dummy::DummyModProvider, traits::provider::Provider};

#[test]
fn api_key_cap_validates() {
    let provider = DummyModProvider::new("dummy");
    let cap = provider.capabilities()
        .iter()
        .find(|o| o.id() == ids::REQUIRES_API_KEY)
        .expect("Api key cap missing");

    let api_cap = cap.as_any()
        .downcast_ref::<ApiKeyCapability<DummyModProvider>>()
        .expect("wrong capability type");

    assert!(api_cap.needs_prompt(None));
    let result = api_cap.on_provided("ABCDEFGHIJKLMNOP");
    assert!(matches!(result, Ok(KeyAction::Store)))
}

#[test]
fn api_key_cap_error_cases() {
    let provider = DummyModProvider::new("dummy");
    let cap = provider
        .capabilities()
        .iter()
        .find(|o| o.id() == ids::REQUIRES_API_KEY)
        .unwrap();
    let api_cap = cap
        .as_any()
        .downcast_ref::<ApiKeyCapability<DummyModProvider>>()
        .unwrap();

    assert!(matches!(
        api_cap.on_provided(""),
        Err(ApiKeyValidationError::Empty)
    ));
    assert!(matches!(
        api_cap.on_provided("SHORT"),
        Err(ApiKeyValidationError::TooShort { min_len : 16 })
    ));
    assert!(matches!(
        api_cap.on_provided("ABCDEFGHIJKLMNOP"),
        Ok(KeyAction::Store)
    ));
}

#[test]
fn api_key_cap_provider_dropped_behaviors() {
    let cap: CapabilityRef = {
        let provider = DummyModProvider::new("dummy");
        provider.capabilities()[0].clone()
    };

    let api_cap = cap
        .as_any()
        .downcast_ref::<ApiKeyCapability<DummyModProvider>>()
        .unwrap();

    // Provider dropped: on_provided should report ProviderError
    assert!(matches!(
        api_cap.on_provided("ABCDEFGHIJKLMNOP"),
        Err(ApiKeyValidationError::ProviderError)
    ));

    // should be false since if we have no provider, we have no prompt
    assert!(!api_cap.needs_prompt(None));
}

#[test]
#[should_panic(expected = "An error occurred while working with the provider.")]
fn api_key_cap_provider_dropped_render_panics() {
    let cap: CapabilityRef = {
        let provider = DummyModProvider::new("dummy");
        provider.capabilities()[0].clone()
    };

    let api_cap = cap
        .as_any()
        .downcast_ref::<ApiKeyCapability<DummyModProvider>>()
        .unwrap();

    let _ = api_cap.render();
}

#[test]
fn capability_cast_ext_helper() {
    let provider = DummyModProvider::new("dummy");
    let cap = provider.capabilities()[0].clone();
    let dyn_ref: &dyn Capability = &*cap;
    let typed = dyn_ref.get::<ApiKeyCapability<DummyModProvider>>();
    assert!(typed.is_some());
    assert_eq!(typed.unwrap().id(), ids::REQUIRES_API_KEY);
}

#[test]
fn capability_builder_api_key_chain() {
    let provider = DummyModProvider::new("builder-test");
    let caps = CapabilityBuilder::new_from_arc(&provider)
        .api_key()
        .finish();

    assert_eq!(caps.len(), 1);
    assert_eq!(caps[0].id(), ids::REQUIRES_API_KEY);
}

struct SimpleCap;
capability!(SimpleCap, "test.simple");

#[test]
fn capability_macro_assigns_id_and_downcast() {
    let cap: CapabilityRef = Arc::new(SimpleCap);
    assert_eq!(cap.id(), "test.simple");
    let dyn_ref: &dyn Capability = &*cap;
    assert!(dyn_ref.get::<SimpleCap>().is_some());
}

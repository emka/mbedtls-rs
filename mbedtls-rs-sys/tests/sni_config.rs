//! Configuration checks for the SNI feature mapping.

#[path = "../gen/config.rs"]
mod config;
#[path = "../gen/features.rs"]
mod features;

fn defines_of(feature: &str) -> &'static [&'static str] {
    features::FEATURE_DEFINES
        .iter()
        .find(|(name, _)| *name == feature)
        .map(|(_, defines)| *defines)
        .unwrap_or_else(|| panic!("no FEATURE_DEFINES entry for {feature}"))
}

#[test]
fn sni_has_its_own_feature_mapping() {
    assert_eq!(defines_of("TLS_SNI"), &["SSL_SERVER_NAME_INDICATION"]);
}

#[test]
fn tls_core_does_not_enable_sni() {
    assert!(!defines_of("TLS_CORE").contains(&"SSL_SERVER_NAME_INDICATION"));
}

#[test]
fn prebuilt_configuration_still_enables_sni() {
    let config = features::prebuilt_features_config();
    assert!(config
        .effective_defines()
        .contains_key("SSL_SERVER_NAME_INDICATION"));
}

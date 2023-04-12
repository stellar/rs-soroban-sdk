use crate::Env;

#[test]
fn default_has_source_account_configured_in_host() {
    let env = Env::default();
    assert!(env.host().source_account().is_some());
}

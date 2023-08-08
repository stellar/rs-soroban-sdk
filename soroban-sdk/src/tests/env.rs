use crate::Env;

#[test]
// Env::default is expected to configure the underlying host with a source
// account in tests so that the Env is configured similarly to how it will be
// configured for real. Some functions in Env have in the past or may now make
// assumptions about a source account being set. This is something small we do
// to make sure we don't accidentally introduce Env functionality that will
// panick in SDK tests.
fn default_has_source_account_configured_in_host() {
    let env = Env::default();
    assert!(env.host().source_account_address().unwrap().is_some());
}

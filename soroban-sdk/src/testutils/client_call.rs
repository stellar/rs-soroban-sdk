use soroban_env_host::auth::AuthorizationManager;

use super::MockAuth;
use crate::{xdr::SorobanAuthorizationEntry, Env};

/// Applies a generated test client's auths for one call, and when dropped
/// restores the auth manager that was in place before the call, including when
/// the call panics.
#[doc(hidden)]
#[must_use]
pub struct ClientCallScope<'a> {
    env: &'a Env,
    auth_manager: Option<AuthorizationManager>,
}

impl<'a> ClientCallScope<'a> {
    pub fn enter(
        env: &'a Env,
        auths: Option<&[SorobanAuthorizationEntry]>,
        mock_auths: Option<&[MockAuth<'_>]>,
        mock_all_auths: bool,
        allow_non_root_auth: bool,
    ) -> Self {
        let scope = Self {
            env,
            auth_manager: (!env.in_contract()).then(|| env.host().snapshot_auth_manager().unwrap()),
        };
        if let Some(auths) = auths {
            env.set_auths(auths);
        }
        if let Some(auths) = mock_auths {
            env.mock_auths(auths);
        }
        if mock_all_auths {
            if allow_non_root_auth {
                env.mock_all_auths_allowing_non_root_auth();
            } else {
                env.mock_all_auths();
            }
        }
        scope
    }
}

impl Drop for ClientCallScope<'_> {
    fn drop(&mut self) {
        if let Some(auth_manager) = self.auth_manager.take() {
            // Continuing with leaked authorization would invalidate the test.
            // If restoration fails during unwinding, the second panic aborts.
            self.env
                .host()
                .set_auth_manager(auth_manager)
                .expect("failed to restore client call state");
        }
    }
}

use soroban_env_host::auth::AuthorizationManager;

use super::MockAuth;
use crate::{xdr::SorobanAuthorizationEntry, Env};

/// Auths that a generated test client applies to each of its calls.
#[doc(hidden)]
#[derive(Clone, Copy, Default)]
pub struct ClientAuths<'a> {
    set_auths: Option<&'a [SorobanAuthorizationEntry]>,
    mock_auths: Option<&'a [MockAuth<'a>]>,
    mock_all_auths: bool,
    allow_non_root_auth: bool,
}

impl<'a> ClientAuths<'a> {
    pub fn set_auths(self, auths: &'a [SorobanAuthorizationEntry]) -> Self {
        Self {
            set_auths: Some(auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
            ..self
        }
    }

    pub fn mock_auths(self, mock_auths: &'a [MockAuth<'a>]) -> Self {
        Self {
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
            ..self
        }
    }

    pub fn mock_all_auths(self) -> Self {
        Self {
            mock_all_auths: true,
            ..Self::default()
        }
    }

    pub fn mock_all_auths_allowing_non_root_auth(self) -> Self {
        Self {
            mock_all_auths: true,
            allow_non_root_auth: true,
            ..Self::default()
        }
    }
}

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
    pub fn enter(env: &'a Env, auths: ClientAuths<'_>) -> Self {
        let scope = Self {
            env,
            auth_manager: (!env.in_contract()).then(|| env.host().snapshot_auth_manager().unwrap()),
        };
        if let Some(set_auths) = auths.set_auths {
            env.set_auths(set_auths);
        }
        if let Some(mock_auths) = auths.mock_auths {
            env.mock_auths(mock_auths);
        }
        if auths.mock_all_auths {
            if auths.allow_non_root_auth {
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

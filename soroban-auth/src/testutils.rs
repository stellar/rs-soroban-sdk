use soroban_sdk::{auth::Context, vec, BytesN, Env, IntoVal, RawVal, Status, Vec};

pub trait EnvAuthUtils {
    /// Calls the special `__check_auth` function of the custom account
    /// contract.
    ///
    /// `__check_auth` can't be called outside of the host-managed `require_auth`
    /// calls. This test utility allows testing custom account contracts without
    /// the need to setup complex contract call trees and enabling the enforcing
    /// auth on the host side.
    ///
    /// This function requires to provide the template argument for error. Use
    /// `soroban_sdk::Status` if `__check_auth` doesn't return a special
    /// contract error and use the error with `contracterror` attribute
    /// otherwise.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contracterror, contractimpl, testutils::BytesN as _, vec, BytesN, Env, Vec, RawVal};
    ///
    /// use soroban_auth::{testutils::EnvAuthUtils, AuthorizationContext};
    ///
    /// #[contracterror]
    /// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
    /// #[repr(u32)]
    /// pub enum NoopAccountError {
    ///     SomeError = 1,
    /// }
    /// struct NoopAccountContract;
    /// #[contractimpl]
    /// impl NoopAccountContract {
    ///
    ///     #[allow(non_snake_case)]
    ///     pub fn __check_auth(
    ///         _env: Env,
    ///         _signature_payload: BytesN<32>,
    ///         signatures: Vec<RawVal>,
    ///         _auth_context: Vec<AuthorizationContext>,
    ///     ) -> Result<(), NoopAccountError> {
    ///         if signatures.is_empty() {
    ///             Err(NoopAccountError::SomeError)
    ///         } else {
    ///             Ok(())
    ///         }
    ///     }
    /// }
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let e: Env = Default::default();
    ///     let account_contract =
    ///         NoopAccountContractClient::new(&e, &e.register_contract(None, NoopAccountContract {}));
    ///     // Non-succesful call of `__check_auth` with a `contracterror` error.
    ///     assert_eq!(
    ///         e.invoke_account_contract_check_auth::<NoopAccountError>(
    ///             &account_contract.contract_id,
    ///             &BytesN::random(&e),
    ///             &vec![&e],
    ///             &vec![&e],
    ///         ),
    ///         // The inner `Result` is for conversion error and will be Ok
    ///         // as long as a valid error type used.
    ///         Err(Ok(NoopAccountError::SomeError))
    ///     );
    ///     // Succesful call of `__check_auth` with a `soroban_sdk::Status`
    ///     // error - this should be compatible with any error type.
    ///     assert_eq!(
    ///         e.invoke_account_contract_check_auth::<soroban_sdk::Status>(
    ///             &account_contract.contract_id,
    ///             &BytesN::random(&e),
    ///             &vec![&e, 0_i32.into()],
    ///             &vec![&e],
    ///         ),
    ///         Ok(())
    ///     );
    /// }
    /// ```
    fn invoke_account_contract_check_auth<E: TryFrom<Status>>(
        &self,
        contract_id: &BytesN<32>,
        signature_payload: &BytesN<32>,
        signatures: &Vec<RawVal>,
        auth_context: &Vec<Context>,
    ) -> Result<(), Result<E, E::Error>>;
}

impl EnvAuthUtils for Env {
    fn invoke_account_contract_check_auth<E: TryFrom<Status>>(
        &self,
        contract_id: &BytesN<32>,
        signature_payload: &BytesN<32>,
        signatures: &Vec<RawVal>,
        auth_context: &Vec<Context>,
    ) -> Result<(), Result<E, E::Error>> {
        let args: Vec<RawVal> = vec![
            &self,
            signature_payload.into_val(self),
            signatures.into_val(self),
            auth_context.into_val(self),
        ];
        let res = self
            .host()
            .call_account_contract_check_auth(contract_id.to_object(), args.to_object());
        match res {
            Ok(rv) => Ok(rv.into_val(self)),
            Err(e) => Err(e.status.try_into()),
        }
    }
}

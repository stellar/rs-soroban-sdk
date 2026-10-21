//! ContractTraitType is an internal trait that code generated for the default
//! fns of a `#[contracttrait]` uses to check that the types the fns name, where
//! the trait is implemented, are the types the trait's fns use.

/// Implemented by each type for itself only, so that a type the trait's fn uses
/// that is not the `Named` type, where the trait is implemented, fails to
/// compile with an error that says which type to import.
#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "the trait's default functions require `{Self}` to be in scope where the trait is implemented",
    label = "`{Named}` here is not `{Self}`",
    note = "import it where the trait is implemented with `use {Self};`, in place of the `{Named}` there"
)]
pub trait ContractTraitType<Named> {}

impl<T> ContractTraitType<T> for T {}

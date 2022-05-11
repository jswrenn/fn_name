#![no_std]
//! Macros that produce the name of the function they're invoked within.
//!
//! ## Uninstantiated Names
//! [`uninstantiated!`]: macro.uninstantiated.html
//! The [`uninstantiated!`] macro produces the name of the surrounding function
//! or method, *without* generics instantiated; e.g.:
//! ```
//! struct GenericType<A>(A);
//!
//! impl<A> GenericType<A> {
//!     fn generic_method<B>(self, _: B) -> &'static str {
//!         fn_name::uninstantiated!()
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(
//!         GenericType(42u8).generic_method(false),
//!         "GenericType<_>::generic_method"
//!     );
//! }
//! ```
//!
//! ## Instantiated Names
//! [`instantiated!`]: macro.instantiated.html
//! The [`instantiated!`] macro produces the name of the surrounding function or
//! method, *including* instantiated generics (if any); e.g.:
//! ```
//! struct GenericType<A>(A);
//!
//! impl<A> GenericType<A> {
//!     fn generic_method<B>(self, _: B) -> &'static str {
//!         fn_name::instantiated!()
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(
//!         GenericType(42u8).generic_method(false),
//!         "GenericType<u8>::generic_method<bool>"
//!     );
//! }
//! ```
//!
//! ## Limitations
//! The expansion of these macros is not (yet) const evaluable; their
//! implementations rely on `core::any::type_name`, which is not a `const fn`.

/// Produces the name of the surrounding function or method, *without* generics
/// instantiated.
///
/// ## Example
/// ```
/// struct GenericType<A>(A);
///
/// impl<A> GenericType<A> {
///     fn generic_method<B>(self, _: B) -> &'static str {
///         fn_name::uninstantiated!()
///     }
/// }
///
/// fn main() {
///     assert_eq!(
///         GenericType(42u8).generic_method(false),
///         "GenericType<_>::generic_method"
///     );
/// }
/// ```
#[macro_export]
macro_rules! uninstantiated {
    () => {{
        struct Here;
        const PREFIX: &str = concat!(module_path!(), "::");
        const SUFFIX: &str = "::Here";
        let here = core::any::type_name::<Here>();
        &here[PREFIX.len()..(here.len() - SUFFIX.len())]
    }}
}

/// Produces the name of the surrounding function or method, *including*
/// instantiated generics (if any).
///
/// ## Example
/// ```
/// struct GenericType<A>(A);
///
/// impl<A> GenericType<A> {
///     fn generic_method<B>(self, _: B) -> &'static str {
///         fn_name::instantiated!()
///     }
/// }
///
/// fn main() {
///     assert_eq!(
///         GenericType(42u8).generic_method(false),
///         "GenericType<u8>::generic_method<bool>"
///     );
/// }
/// ```
#[macro_export]
macro_rules! instantiated {
    () => {{
        fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
            core::any::type_name::<T>()
        }
        const PREFIX: &str = concat!(module_path!(), "::");
        const SUFFIX: &str = "::{{closure}}";
        let here = &type_name_of_val(&||{});
        &here[PREFIX.len()..(here.len() - SUFFIX.len())]
    }}
}

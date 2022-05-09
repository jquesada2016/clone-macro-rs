//! A simple macro to make cloning data before passing it into a `move` closure or block.
//!
//! This macro is intentionally designed to be compatible with
//! `rustfmt` formatting.
//!
//! You can use this macro throughout your crate without needing to explicitly
//! import it every time as follows:
//! ```ignore
//! #[macro_use]
//! extern crate clone_macro;
//!
//! /* ... */
//!
//! clone!(/* ... */);
//! ```
//!
//! Otherwise, you can `use` it as normal.
//! ```rust
//! use clone_macro::clone;
//!
//! /* ... */
//! clone!(/* ... */);
//! ```
//!
//! # Syntax
//! The `clone!` macro takes a comma separated list of either one of two forms
//! which can have an optional `mut` prefix modifier, followed by an arbitrary
//! expression.
//!
//! For example, the following is a valid call
//! ```rust
//! # use clone_macro::clone;
//!
//! let a = 1;
//! let b = 2;
//!
//! clone!([mut a, b], ());
//! ```
//!
//! and desugars down to:
//! ```rust
//! let a = 1;
//! let b = 2;
//!
//! {
//!     let mut a = a.clone();
//!     let b = b.clone();
//!
//!     ()
//! };
//! ```
//!
//! The clone list can also take a second form, which is an arbitrary expression
//! followed by `as` and the name of the variable. For example:
//! ```rust
//! # use clone_macro::clone;
//!
//! let s = "Hello, there!";
//!
//! clone!([{ s.len() } as len], move || {
//!     assert_eq!(len, "Hello, there!".len());
//! });
//! ```
//!
//! The above desugars into:
//! ```rust
//! # use clone_macro::clone;
//!
//! let s = "Hello, there!";
//!
//! {
//!     let len = "Hello, there!".len();
//!
//!     move || {
//!         assert_eq!(len, "Hello, there!".len());
//!     }
//! };
//! ```
//!
//! This macro is most useful when the second argument is a closure, and is what
//! it is intended to work with, though not strictly so.
//!
//! All forms mentioned above can be mixed and matched, including adding a `mut` modifier
//! for the second form as:
//! ```rust,ignore
//! mut { $expr } as $ident
//! ```
//!
//! # Examples
//! ## Basic Usage
//!
//! ```rust
//! use clone_macro::clone;
//!
//! let s = "You are a beautiful being!".to_string();
//!
//! let c = clone!([s], move || {
//!     println!("{s}");
//! });
//!
//! c();
//!
//! // `s` wasn't directly moved, rather, cloned first, then moved; therefore,
//! // we can still use `s`
//! assert_eq!(s.as_str(), "You are a beautiful being!");
//! ```
//!
//! We can also declare the cloned `move` as `mut`:
//! ```rust
//! use clone_macro::clone;
//!
//! let a = 7;
//! let b = 0;
//! let d = 12;
//!
//! let mut c = clone!([a, mut b, d], move || {
//!     b = 42 - a - d;
//!
//!     println!("a + b + d = {}", a + b + d);
//! });
//!
//! c();
//!
//! assert_eq!(a, 7);
//! assert_eq!(b, 0);
//! assert_eq!(d, 12);
//! ```
//!
//! ## Advanced Usage
//! We can clone arbitrary expressions:
//! ```rust
//! use clone_macro::clone;
//!
//! struct MyStruct {
//!     some_field: String,
//! }
//!
//! let s = MyStruct {
//!     some_field: "Beyond measure.".to_string(),
//! };
//!
//! let mut c = clone!([{ s.some_field } as some_field, mut { s.some_field } as mut_some_field], move || {
//!     mut_some_field.clear();
//!
//!     assert!(mut_some_field.is_empty());
//!
//!     assert_eq!(some_field.as_str(), "Beyond measure.");
//! });
//!
//! c();
//!
//! assert_eq!(s.some_field.as_str(), "Beyond measure.");
//! ```

/// Please see the crate documentation for syntax and examples, but in a jist, the
/// syntax is as follows:
/// ```ignore
/// clone!([$($(mut)? $FORM)*], $expr);
/// ```
///
/// where `$FORM` is one of either:
/// - `ident`
/// - `{ $expr } as ident`
#[macro_export]
macro_rules! clone {
    () => {};
    ([$($tt:tt)*], $expr:expr) => {{
        clone!($($tt)*);

        $expr
    }};
    ($(,)? mut { $expr:expr } as $ident:ident $($tt:tt)*) => {
        let mut $ident = ::core::clone::Clone::clone(&$expr);
        clone!($($tt)*);
    };
    ($(,)? mut $ident:ident $($tt:tt)*) => {
        let mut $ident = ::core::clone::Clone::clone(&$ident);
        clone!($($tt)*);
    };
    ($(,)? { $expr:expr } as $ident:ident $($tt:tt)*) => {
        let $ident = ::core::clone::Clone::clone(&$expr);
        clone!($($tt)*);
    };
    ($(,)? $ident:ident $($tt:tt)*) => {
        let $ident = ::core::clone::Clone::clone(&$ident);
        clone!($($tt)*);
    };
    ($(,)?) => {};
}

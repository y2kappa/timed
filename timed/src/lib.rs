//! # Timed
//!
//! Macros for measuring function execution.
//! ```
//! #[timed::timed]
//! fn add(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! ```
//! It will output:
//! ```
//! // function=add duration=112ns
//! ```

extern crate proc_macro;

use quote::quote;
use syn::ItemFn;

/// Times the execution of the function
///
/// # Examples
///
/// ```
/// #[timed::timed]
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
///
/// It will output:
/// ```
/// // function=add duration=112ns
/// ```
///
/// The implementation renames the given function
/// by sufixing it with `_impl_` so in this case
/// you will have `fn _impl_add(x:i32, y:i32)`
/// and creates a new function with the original name.
///
/// Thie is the final output after the macro expands:
/// ```
/// fn _impl_add(x:i32, y:i32) -> i32 {
///     x + y
/// }
///
/// fn add(x: i32, y:i32) -> i32 {
///     use std::time::Instant;
///     let _start = Instant::now();
///     let res = _impl_add(x, y);
///     println!("function={} duration={:?}", "add", _start.elapsed());
///     res
/// }
/// ```
///
/// Currently it works with any function type.
///
/// Work in progress to add attributes
///
///
#[proc_macro_attribute]
pub fn timed(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);
    let attrs = &input.attrs;
    let vis: &syn::Visibility = &input.vis;
    let sig: &syn::Signature = &input.sig;
    let body: &Box<syn::Block> = &input.block;
    let name = &input.sig.ident;

    let result = quote! {
        #(#attrs)*
        #vis #sig {
           let start = std::time::Instant::now();
           let res = { #body };
           println!("function={} duration={:?}", stringify!(#name), start.elapsed());
           res
        }
    };

    let res: proc_macro::TokenStream = result.into();
    proc_macro::TokenStream::from(res)
}

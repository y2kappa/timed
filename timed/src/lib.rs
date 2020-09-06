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


/// Times the execution of the function
///
/// # Examples
///
/// ```
/// use timed::timed;
///
/// #[timed]
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
///
/// #[timed(printer = "println!")]
/// async fn google()  {
///     // reqwest::get("https://google.com").await;
/// }
/// ```
///
/// ```compile_fail
/// #[timed(printer = "info!")]
/// fn add_info(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
///
/// ```compile_fail
/// #[tokio::main]
/// #[timed]
/// async fn main() {
///     reqwest::get("https://google.com").await;
/// }
///
/// ```
///
/// It will output:
/// ```
/// // function=add duration=112ns
/// ```
///

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    printer: Option<String>,
}

fn printer(options: &MacroArgs, function_name: &syn::Ident) -> proc_macro2::TokenStream {
    let (printer, needs_bang) = match &options.printer {
        Some(printer) => {
            if printer.ends_with('!') {
                (&printer[..&printer.len() - 1], true)
            } else {
                (printer.as_str(), false)
            }
        }
        None => ("println", true),
    };

    let bang = if needs_bang {
        Some(syn::token::Bang(Span::call_site()))
    } else {
        None
    };

    let printer = syn::Ident::new(printer, Span::call_site());

    quote! {
        #printer#bang("function={} duration={:?}", stringify!(#function_name), start.elapsed());
    }
}

#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    // debug!("Args {:?}", args);

    let options = syn::parse_macro_input!(args as AttributeArgs);
    let function = syn::parse_macro_input!(input as ItemFn);

    // Parse options
    let options: MacroArgs = match MacroArgs::from_list(&options) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    // debug!("Parsed options {:?}", options);

    // Parse function sig
    let ItemFn {
        attrs,
        vis,
        sig,
        block: body,
        ..
    } = &function;

    let name = &function.sig.ident;
    let printer = printer(&options, &name);

    let result = quote! {
        #(#attrs)*
        #vis #sig {
           let start = std::time::Instant::now();
           let res = { #body };
           #printer
           res
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

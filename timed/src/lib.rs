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
//! Times the execution of the function
//!
//! # Examples
//!
//! ```
//! use timed::timed;
//!
//! #[timed]
//! fn add(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//!
//! #[timed(printer = "println!")]
//! async fn google()  {
//!     // reqwest::get("https://google.com").await;
//! }
//! ```
//!
//! ```ignore
//! #[timed(printer = "info!")]
//! fn add_info(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! ```
//!
//! ```ignore
//! #[tokio::main]
//! #[timed]
//! async fn main() {
//!     reqwest::get("https://google.com").await;
//! }
//!
//! ```

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    printer: Option<String>,
    #[darling(default)]
    tracing: Option<bool>,
}

use proc_macro2::TokenStream as Code;

fn tracing(options: &MacroArgs, function_name: &str) -> Option<(Code, Code)> {
    if let Some(true) = options.tracing {
        let begin = quote! {
            let _tracing_begin = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros();
        };
        let end = quote! {
            let _tracing_end = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros();
        };
        Some((begin, end))
    } else {
        None
    }
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

/// Macro that times your function execution.
#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    //! ```text
    //! Call this using #[timed]
    //! It will print by default with `println!`
    //! If you want to change the printer you can use #[timed(printer = "info!")]
    //! or any other macro or function that takes in a String.`
    //! ```
    //!
    //! ```ignore
    //! #[timed(printer = "println!")]
    //! ```
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
    let (tracing_begin, tracing_end) = tracing(&options, &name);

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

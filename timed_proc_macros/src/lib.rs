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

fn codegen_tracing(options: &MacroArgs, function_name: &str) -> (Option<Code>, Option<Code>) {
    if let Some(true) = options.tracing {
        let begin = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                timed::Trace::collect(timed::Hop { ph: timed::Phase::Start, name: format!("{}::{}", module_path, #function_name), ts});
            }
        };
        let end = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                timed::Trace::collect(timed::Hop { ph: timed::Phase::Finish(elapsed), name: format!("{}::{}", module_path, #function_name), ts});
            }
        };
        (Some(begin), Some(end))
    } else {
        (None, None)
    }
}

fn codegen_duration(
    printer: &proc_macro2::TokenStream,
    function_name: &str,
) -> proc_macro2::TokenStream {
    quote! {
        #printer("function={} duration={:?}", format!("{}::{}", module_path, #function_name), elapsed);
    }
}

fn codegen_printer(options: &MacroArgs) -> proc_macro2::TokenStream {
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
        #printer#bang
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
    let name = function.sig.ident.to_string();
    let printer = codegen_printer(&options);
    let print_duration = codegen_duration(&printer, name.as_str());
    let (tracing_begin, tracing_end) = codegen_tracing(&options, name.as_str());

    let result = quote! {
        #(#attrs)*
        #vis #sig {
           let module_path = std::module_path!();
           #tracing_begin
           let start = std::time::Instant::now();
           let res = { #body };
           let elapsed = start.elapsed();
           #print_duration
           #tracing_end
           res
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

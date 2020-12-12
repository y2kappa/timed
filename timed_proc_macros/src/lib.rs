#![allow(unused_imports)]

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
struct TracingArgs {
    #[darling(default)]
    enabled: Option<bool>,
    #[darling(default)]
    main: Option<String>,
}

#[derive(Debug, FromMeta)]
struct DurationArgs {
    #[darling(default)]
    disabled: bool,
    #[darling(default)]
    printer: Option<String>,
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    tracing: Option<TracingArgs>,
    #[darling(default)]
    duration: Option<DurationArgs>,
}

use proc_macro2::TokenStream as Code;

fn codegen_tracing(options: &MacroArgs, function_name: &str) -> (Option<Code>, Option<Code>) {
    if let Some(_) = options.tracing {
        let begin = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                timed::Trace::record(timed::Hop {
                    function_name: format!("{}::{}", __timed_module_path, #function_name),
                    timestamp: ts,
                    phase: timed::Phase::Start
                });
            }
        };
        let end = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                timed::Trace::record(timed::Hop {
                    function_name: format!("{}::{}", __timed_module_path, #function_name),
                    timestamp: ts,
                    phase: timed::Phase::Finish(__timed_elapsed)
                });
            }
        };
        (Some(begin), Some(end))
    } else {
        (None, None)
    }
}

fn codegen_printer(options: &Option<String>) -> proc_macro2::TokenStream {
    let (printer, needs_bang) = match &options {
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

fn codegen_duration(options: &MacroArgs, function_name: &String) -> (Code, Code) {
    // Generate printer
    let printer_options = match &options.duration {
        Some(options) => &options.printer,
        None => &None,
    };
    let printer = codegen_printer(&printer_options);

    // Decide if we generate duration at all
    let disabled = match options.duration {
        Some(DurationArgs { disabled, .. }) => disabled,
        _ => true,
    };

    if let Some(duration_args) = &options.duration {
        println!("{:?}", duration_args);
    }

    let duration_begin = quote! {
        let __timed_start = std::time::Instant::now();
    };

    let duration_end = if disabled {
        quote! {
            let __timed_elapsed = __timed_start.elapsed();
        }
    } else {
        quote! {
            let __timed_elapsed = __timed_start.elapsed();
            #printer("function={} duration={:?}", #function_name, __timed_elapsed);
        }
    };

    (duration_begin, duration_end)
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
    //! #[timed(duration(printer = "println!"))]
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

    let (duration_begin, duration_end) = codegen_duration(&options, &name);
    let (tracing_begin, tracing_end) = codegen_tracing(&options, name.as_str());

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            let __timed_module_path = std::module_path!();

            #tracing_begin
            #duration_begin
            let result = { #body };

            #duration_end
            #tracing_end

            result
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

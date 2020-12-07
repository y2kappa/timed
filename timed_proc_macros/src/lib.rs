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
struct MacroArgs {
    #[darling(default)]
    tracing: Option<TracingArgs>,
    // An idea printer could be a 'sink' so it can either println!() each or
    // we can collect the data into a vector
    #[darling(default)]
    printer: Option<String>,
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

fn codegen_duration(
    printer: &proc_macro2::TokenStream,
    function_name: &String,
) -> proc_macro2::TokenStream {
    quote! {
        #printer("function={} duration={:?}", #function_name, __timed_elapsed);
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
    let print_duration = codegen_duration(&printer, &name);
    let (tracing_begin, tracing_end) = codegen_tracing(&options, name.as_str());

    let result = quote! {
        #(#attrs)*
        #vis #sig {
           let __timed_module_path = std::module_path!();

           #tracing_begin

           let __timed_start = std::time::Instant::now();
           let __timed_res = { #body };
           let __timed_elapsed = __timed_start.elapsed();

           #tracing_end
           #print_duration

           __timed_res
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

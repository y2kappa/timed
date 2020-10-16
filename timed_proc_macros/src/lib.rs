
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

// struct Tracing;
// impl Drop for Tracing {
//     fn drop(&mut self) {
//         let traces = TRACES.lock().unwrap();
//         println!("Begin Dumping traces:\n-----");
//         println!("[");
//         for i in 0..traces.len() {
//             println!("    {}{}", traces[i], if i == traces.len() - 1 { "" } else { ","});
//         }
//         println!("]");
//         println!("-----\nEnd Dumping traces");
//     }
// }


use proc_macro2::TokenStream as Code;

fn codegen_tracing(options: &MacroArgs, function_name: &str) -> (Option<Code>, Option<Code>) {
    if let Some(true) = options.tracing {
        let begin = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                let trace = format!("{{ \"pid\": 0, \"ts\": {},  \"ph\": \"B\", \"name\": \"{}\" }}", ts, #function_name);
                timed::collect(timed::Action::Collect(trace.clone()));
            }
        };
        let end = quote! {
            {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                let trace = format!("{{ \"pid\": 0, \"ts\": {}, \"ph\": \"E\", \"name\": \"{}\" }}", ts, #function_name);
                timed::collect(timed::Action::Collect(trace.clone()));
            }
        };
        (Some(begin), Some(end))
    } else {
        (None, None)
    }
}

fn codegen_duration(printer: &proc_macro2::TokenStream, function_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        #printer("function={} duration={:?}", stringify!(#function_name), start.elapsed());
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

    let name = &function.sig.ident;
    let printer = codegen_printer(&options);
    let print_duration = codegen_duration(&printer, &name);
    let (tracing_begin, tracing_end) = codegen_tracing(&options, &name.to_string());

    let result = quote! {
        #(#attrs)*
        #vis #sig {
           #tracing_begin
           let start = std::time::Instant::now();
           let res = { #body };
           #print_duration
           #tracing_end
           res
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

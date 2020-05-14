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
//! function=add duration=112ns
//! ```

extern crate proc_macro;

use proc_macro2::{Span, TokenTree,
    Delimiter::Brace, Delimiter::Parenthesis,
    Punct, Spacing};
use quote::quote;

struct FunctionParser {
    name: Option<String>,
    arguments: Option<proc_macro2::Group>,
    parameters: Vec<String>,
    return_type: Option<String>,
    body: Option<proc_macro2::Group>
}

struct Function {
    impl_name: proc_macro2::Ident,
    orig_name: proc_macro2::Ident,

    /// At signature level
    /// In this case:
    /// fn add (x: i32, y: i32) { x + y } add(2,3)
    /// arguments are (x: i32, y: i32)
    arguments: proc_macro2::Group,

    /// At caller level
    /// In this case:
    /// fn add (x: i32, y: i32) { x + y } add(2,3)
    /// parameters are (2, 3)
    parameters: Vec<proc_macro2::Ident>,

    return_type: Option<proc_macro2::Ident>,

    return_arrow_line: Option<Punct>,
    return_arrow_gt: Option<Punct>,


    body: Option<proc_macro2::Group>
}

impl FunctionParser {
    fn new() -> Self {
        FunctionParser {
            name: None,
            arguments: None,
            parameters: vec![],
            return_type: None,
            body: None
        }
    }

    fn validate(&self) {
        if self.name.is_none() {
            panic!("Invalid input")
        }
    }

}

impl Function {
    fn from(parser: &FunctionParser) -> Self {

        let orig_name = parser.name.as_ref().unwrap();
        let impl_name = format!("_impl_{}", orig_name);

        Function {

            orig_name: proc_macro2::Ident::new(&orig_name, Span::call_site()),

            impl_name: proc_macro2::Ident::new(&impl_name, Span::call_site()),

            arguments: parser.arguments.as_ref().unwrap().clone(),

            parameters: parser.parameters.iter()
                .map(|x| proc_macro2::Ident::new(&x, Span::call_site()))
                .collect(),

            return_type: match &parser.return_type {
                None => None,
                Some(rt) => Some(proc_macro2::Ident::new(&rt, Span::call_site())),
            },

            return_arrow_line: match &parser.return_type {
                None => None,
                Some(_rt) => Some(Punct::new('-', Spacing::Joint)),
            },
            return_arrow_gt: match &parser.return_type {
                None => None,
                Some(_rt) => Some(Punct::new('>', Spacing::Alone)),
            },

            body: parser.body.clone()
        }
    }
}

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
/// function=add duration=112ns
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
///     println!("function={} duration={:?}", "add", start.elapsed());
///     res
/// }
/// ```
///
/// Currently it works with functions passed by value only, or no
/// parameters at all.
///
/// Work in progress to cover more cases.
///
///

#[proc_macro_attribute]
pub fn timed(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input = proc_macro2::TokenStream::from(input);

    let mut parser = FunctionParser::new();

    // Parsing the token tree
    for item in input.into_iter() {
        match item {

            // This could be function params or function body
            // Only these are wrapped around parantheses

            TokenTree::Group(ref g) => {
                match g.delimiter() {

                    // The first time we encounter a group
                    // surrounded by braces it means it's the body.
                    // We could be wrong in some cases, but we need to
                    // develop iteratively and discover usecases.

                    Brace => {
                        parser.body = Some(g.clone());
                    },

                    // The first time we encounter a group
                    // surrounded by round parantheses it means it's the arguments.
                    // We could be wrong in some cases, but we need to
                    // develop iteratively and discover usecases.
                    Parenthesis => {

                        // We keep them as tokens so we can replicate the
                        // function, but also to we can call the function.

                        parser.arguments = Some(g.clone());
                        for (i, item) in g.stream().into_iter().enumerate() {

                            // it's like this
                            // (x: i32, y: i32)
                            // the named parameter is at every 4th token

                            if i % 4 == 0 {
                                parser.parameters.push(item.to_string());
                            }
                        }
                    },
                    other => println!("Unmatched {:?}", other)
                };
            },

            // This could be either function name or reserved types
            // such as return type i32, String, etc

            TokenTree::Ident(ref i) => {
                match i.to_string() {
                    s if s == "fn" => { },
                    other => {

                        if parser.name.is_none() {
                            parser.name = Some(other.clone());
                        } else {
                            parser.return_type = Some(other.clone());
                        }
                    },
                }
            },
            _other => {
                // println!("{:?}", _other);
                // Punctuation or literal
            }
        }
    }

    parser.validate();

    let function = Function::from(&parser);
    let debug_name = parser.name.as_ref().unwrap();

    let arguments = function.arguments;
    let parameters = function.parameters;
    let orig_name = function.orig_name;
    let impl_name = function.impl_name;
    let return_type = function.return_type;
    let return_arrow_line = function.return_arrow_line;
    let return_arrow_gt = function.return_arrow_gt;
    let body = function.body;

    let result = quote! {

        fn #impl_name#arguments #return_arrow_line #return_arrow_gt #return_type #body

        fn #orig_name#arguments #return_arrow_line #return_arrow_gt #return_type {
            use std::time::Instant;
            let start = Instant::now();
            let res = #impl_name(#(#parameters),*);
            println!("function={} duration={:?}", #debug_name, start.elapsed());
            res
        }
    };

    // println!("Generating code {}", &result.to_string());

    let res : proc_macro::TokenStream = result.into();
    proc_macro::TokenStream::from(res)
}
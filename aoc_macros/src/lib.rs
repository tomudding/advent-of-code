extern crate aoc_function_registry;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};
use darling::{FromMeta, Error};
use darling::ast::NestedMeta;

#[derive(Debug, FromMeta)]
struct AoCAttributes {
    year: LitStr,
    day: LitStr,
    part: LitStr,
    function: Option<LitStr>,
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let _input = parse_macro_input!(input as ItemFn);
    let _args = match AoCAttributes::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    
    let fn_name = &_input.sig.ident;
    let registry_key: String = if let Some(function) = _args.function {
        format!("{}_{}_{}_{}", _args.year.value(), _args.day.value(), _args.part.value(), function.value())
    } else {
        format!("{}_{}_{}", _args.year.value(), _args.day.value(), _args.part.value())
    };

    let expanded = quote! {
        #_input

        const _: () = {
            #[used]
            #[allow(non_upper_case_globals)]
            #[link_section = ".init_array"]
            static __REGISTER_FN: fn() = || {
                let mut registry = get_registry().lock().unwrap();
                registry.insert(#registry_key.to_string(), #fn_name as fn() -> String);
            };
        };
    };

    TokenStream::from(expanded)
}

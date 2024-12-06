extern crate aoc_shared_functions;

use darling::{FromMeta, Error};
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::fs::DirEntry;
use std::path::PathBuf;
use proc_macro2::Span;
use syn::{parse_macro_input, Ident, ItemFn, LitStr};

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

#[proc_macro]
pub fn include_year_modules(input: TokenStream) -> TokenStream {
    let year = parse_macro_input!(input as LitStr).value();

    let mut modules = Vec::new();
    for entry in std::fs::read_dir(format!("src/{}", year)).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "rs" {
            let file_name: &str = path.file_stem().unwrap().to_str().unwrap();
            modules.push((file_name.to_string(), year.clone()));
        }
    }

    let module_declarations = modules.iter().map(|(module, year)| {
        let ident: Ident = Ident::new(&format!("year{}_{}", year, module), Span::call_site());
        let path: String = format!("{}/{}.rs", year, module);

        quote! {
            #[path = #path]
            pub mod #ident;
        }
    });

    let expanded = quote! {
        #(#module_declarations)*
    };

    TokenStream::from(expanded)
}
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::fs::DirEntry;
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

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
        let ident: Ident = Ident::new(&format!("year{}_{}", year, module), proc_macro2::Span::call_site());
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

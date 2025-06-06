use syn::{parse_macro_input, Attribute};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn has_serde_default(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;

            if let Ok(nested) = attr.parse_args_with(parser) {
                for nested_meta in nested {
                    if let syn::Meta::Path(path) = nested_meta {
                        if path.is_ident("default") {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

#[proc_macro_derive(SupernovaConfig, attributes(supernova_config))]
pub fn supernova_config_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut config_name = None;

    for attr in &ast.attrs {
        if attr.path().is_ident("supernova_config") {
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;

            if let Ok(nested) = attr.parse_args_with(parser) {
                for nested_meta in nested {
                    if let syn::Meta::NameValue(nv) = nested_meta {
                        if nv.path.is_ident("name") {
                            if let syn::Expr::Lit(expr_lit) = nv.value {
                                if let syn::Lit::Str(litstr) = expr_lit.lit {
                                    config_name = Some(litstr.value());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let default_arm = if has_serde_default(&ast.attrs) {
        quote! {
            Ok(Self::default())
        }
    } else {
        quote! {
            Err(
                "Unable to load config from file and there are no default values for this config. ".to_string() +
                "Consider adding #[serde(default)] for this struct and implementing the Default trait."
            )
        }
    };

    let config_name = config_name.unwrap_or("base".to_string());    
    let ident = &ast.ident;

    let quoted_input = quote! {
        use std::path::PathBuf;
        use std::fs::OpenOptions;
        use std::io::Write;

        use serde_json;

        impl #ident {
            pub fn save_defaults_if_not_exists() -> Result<(), std::io::Error> {
                let mut config = Self::load().unwrap();
                config.save_if_not_exists()?;
                Ok(())
            }

            pub fn save_if_not_exists(self) -> Result<(Self), std::io::Error> {
                let path = std::env::var("SUPERNOVA_CONFIG_PATH").unwrap_or(".".into());
                let path = PathBuf::from(path);                
                let path = path.join(format!("sn-config-{}.json", #config_name));

                let mut file = std::fs::OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(path)?;

                file.write_all(serde_json::to_string_pretty(&self).unwrap().as_bytes())?;
                Ok(self)
            }

            pub fn load() -> Result<Self, String> {
                let path = std::env::var("SUPERNOVA_CONFIG_PATH").unwrap_or(".".into());
                let path = PathBuf::from(path);                
                let path = path.join(format!("sn-config-{}.json", #config_name));

                match std::fs::File::open(path) {
                    Ok(file) => {
                        match serde_json::from_reader(file) {
                            Ok(config) => Ok(config),
                            Err(e) => Err(format!("Error loading config from file: {}", e)),
                        }
                    }
                    Err(e) => {
                        #default_arm
                    }
                }
            }
        }
    };

    quoted_input.into()
}

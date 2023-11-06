extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
};

#[proc_macro_derive(MergeCfg)]
pub fn merge_cfg_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;

    let mut merge_handle = quote! {};
    match ast.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let field_ident = field.ident.as_ref().unwrap();
                    let field_name = field_ident.to_string();
                    let field_type = field.ty.clone();
                    // cast and set value
                    match &field_type {
                        // currently just support Vec<T>
                        Type::Path(p) if p.path.segments.first().unwrap().ident.eq("Vec") => {
                            if let PathArguments::AngleBracketed(generic_args) =
                                &p.path.segments.first().unwrap().arguments
                            {
                                let args = &generic_args.args;
                                if args.len().eq(&1) {
                                    if let GenericArgument::Type(Type::Path(ap)) = &args[0] {
                                        // get type string
                                        let ty = ap.path.get_ident().unwrap().to_string();
                                        // get type identity
                                        let ty_ident = Ident::new(&ty, Span::call_site().into());
                                        merge_handle = quote! {
                                            #merge_handle
                                            #field_name => {
                                                self.#field_ident = v.iter().map(|s| s.parse::<#ty_ident>().unwrap()).collect()
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        _ => {
                            merge_handle = quote! {
                                #merge_handle
                                #field_name => {
                                    self.#field_ident = v.first().unwrap().parse::<#field_type>().unwrap()
                                }
                            };
                        }
                    };
                }
            }
            _ => panic!("MergeCfg can only be derived on structs with named fields"),
        },
        _ => panic!("MergeCfg can only be derived on structs"),
    }

    // final expanded code
    quote! {
        impl #struct_ident {
            pub fn merge_cfg(&mut self) {
                let args: Vec<String> = ::std::env::args().collect();
                if args.len() > 1 {
                    let mut map = ::std::collections::HashMap::<String, Vec<String>>::new();
                    for arg in args.iter().skip(1) {
                        if let Some((k, v)) = arg.split_once('=') {
                            map.entry(k.to_string()).or_default().push(v.to_string());
                        }
                    }
                    map.iter().for_each(|(k, v)| {
                        match k.as_str() {
                            #merge_handle
                            _ => {},
                        }
                    });
                }
            }
        }
    }
    .into()
}

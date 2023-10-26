extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                    merge_handle = quote! {
                        #merge_handle
                        #field_name => {
                            self.#field_ident = kv[1].parse::<#field_type>().unwrap()
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
                        for arg in args.iter().skip(1) {
                            let kv: Vec<&str> = arg.split('=').collect();
                            if kv.len() == 2 {
                                match kv[0] {
                                    #merge_handle
                                    _ => {}
                                }
                            }
                        }
                    }
            }
        }
    }
    .into()
}

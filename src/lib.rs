#![crate_type = "proc-macro"]

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[derive(darling::FromMeta, Default)]
struct MacroOptions {
    #[darling(default)]
    red_zone: Option<usize>,
    #[darling(default)]
    stack_size: Option<usize>,
}

#[proc_macro_attribute]
pub fn maybe_grow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let input_attr = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let input_attr = MacroOptions::from_list(&input_attr).unwrap();

    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_outputs = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_generics = &input_fn.sig.generics;

    let red_zone = input_attr.red_zone.unwrap_or(1024 * 32);
    let stack_size = input_attr.stack_size.unwrap_or(1024 * 1024);

    let expanded = quote! {
      #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_outputs {
        stacker::maybe_grow(#red_zone, #stack_size, || {
          #fn_block
        })
      }
    };

    expanded.into()
}

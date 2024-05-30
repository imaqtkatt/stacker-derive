#![crate_type = "proc-macro"]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn maybe_grow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_outputs = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_generics = &input_fn.sig.generics;

    let expanded = quote! {
      #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_outputs {
        stacker::maybe_grow(1024 * 32, 1024 * 1024, || {
          #fn_block
        })
      }
    };

    expanded.into()
}

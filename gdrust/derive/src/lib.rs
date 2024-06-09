use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl};

#[proc_macro_attribute]
pub fn gen_debug(_attr: TokenStream, tok: TokenStream) -> TokenStream {
    let mut input_impl = parse_macro_input!(tok as ItemImpl);
    let mut ret = vec![];
    let mut should_be_called = vec![];
    for item in input_impl.items.iter_mut() {
        if let ImplItem::Fn(function) = item {
            //首先是函数的解析
            let flag = function
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("debug"));
            if flag {
                function.attrs.retain(|item| !item.path().is_ident("debug"));
                let name = function.sig.ident.clone();
                should_be_called.push(name);
            }
        }
        ret.push(item);
    }
    let debug_check_func: TokenStream = quote! {
    #[cfg(debug_assertions)]
             fn debug_check(&mut self) {
                 #(
                     self.#should_be_called();
                 )*
             }

    }
    .into();
    let func = parse_macro_input!(debug_check_func as ImplItem);
    input_impl.items.push(func);
    let ret: TokenStream = quote::quote! {
        #input_impl
    }
    .into();
    ret
}

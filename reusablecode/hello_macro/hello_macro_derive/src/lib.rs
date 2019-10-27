extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_hello_macro(&ast);

    gen.parse().unwrap()
}

fn impl_hello_macro(ast: &syn::DriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello, macro I'm a {}", stringify!(#name));
            }
        }
    }
}
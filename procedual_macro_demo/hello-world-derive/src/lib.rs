extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let gen = impl_hello_world(&ast);
  TokenStream::from(gen)
}

fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let expand = quote!{
    impl HelloWorld for #name {
      fn hello_world(&self) {
        println!("hello {}", stringify!(#name));
      }
    }

    impl #name {
      fn hello_world2(&self) {
        println!("hello2 {}", stringify!(#name));
      }
    }
  };
  expand.into()
}

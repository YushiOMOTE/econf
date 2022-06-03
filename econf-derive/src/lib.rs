extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(LoadEnv)]
pub fn load_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let content = content(&name, &input.data);

    let expanded = quote! {
        impl #impl_generics ::econf::LoadEnv for #name #ty_generics #where_clause {
            fn load(self, path: &str, dup: &mut ::std::collections::HashSet<String>) -> Self {
                #content
            }
        }
    };

    TokenStream::from(expanded)
}

fn content(name: &Ident, data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let fnames0 = fields.named.iter().map(|f| &f.ident);
                let fnames1 = fields.named.iter().map(|f| &f.ident);
                let fnames2 = fields.named.iter().map(|f| &f.ident);
                quote! {
                    Self { #(
                        #fnames0: self.#fnames1.load(&(path.to_owned() + "_" + stringify!(#fnames2)), dup),
                    )* }
                }
            }
            Fields::Unnamed(fields) => {
                let indices0 = (0..fields.unnamed.len()).map(|i| syn::Index::from(i));
                let indices1 = (0..fields.unnamed.len()).map(|i| syn::Index::from(i));
                quote! {
                    #name ( #(
                        self.#indices0.load(&(path.to_owned() + "_" + &#indices1.to_string()), dup),
                    )* )
                }
            }
            Fields::Unit => quote!(#name),
        },
        Data::Enum(data) => {
            data.variants.iter().for_each(|f| match f.fields {
                Fields::Named(_) => panic!("Enum variant with named fields are not supported"),
                Fields::Unnamed(_) => panic!("Enum variant with unnamed fields are not supported"),
                Fields::Unit => {}
            });

            let enums0 = data.variants.iter().map(|_| &name);
            let enums1 = data.variants.iter().map(|f| &f.ident);
            let enums2 = data.variants.iter().map(|f| &f.ident);

            quote! {
                match String::default().load(path, dup).as_ref() {
                    #(
                        stringify!(#enums1) => #enums0::#enums2,
                    )*
                    "" => self,
                    x => {
                        error!("econf: couldn't find variant: {}", x);
                        self
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!("Unions are not supported"),
    }
}

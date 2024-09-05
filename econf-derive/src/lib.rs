extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, LitStr, Variant};

#[proc_macro_derive(LoadEnv, attributes(econf))]
pub fn load_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let content = content(&name, &input.data);

    let expanded = quote! {
        impl #impl_generics ::econf::LoadEnv for #name #ty_generics #where_clause {
            fn load(self, path: &str, loader: &mut ::econf::Loader) -> Self {
                #content
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_skip(f: &Field) -> bool {
    f.attrs.iter().any(|attr| {
        if attr.path().is_ident("econf") {
            if let Ok(args) = attr.parse_args::<Ident>() {
                return args == "skip";
            }
        }

        false
    })
}

fn find_renaming(attrs: &[Attribute]) -> Option<String> {
    let mut rename = None;
    for attr in attrs {
        if attr.path().is_ident("econf") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let s: LitStr = meta.value()?.parse()?;
                    rename = Some(s.value());
                }

                Ok(())
            })
            .expect("failed to parse nested meta");
        }
    }

    rename
}

fn find_field_renaming(f: &Field) -> Option<String> {
    find_renaming(&f.attrs)
}

fn find_variant_renaming(v: &Variant) -> Option<String> {
    find_renaming(&v.attrs)
}

fn content(name: &Ident, data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    if is_skip(f) {
                        return quote! {
                            #ident: self.#ident,
                        };
                    }
                    match find_field_renaming(f) {
                        Some(overwritten_name) => quote! {
                            #ident: self.#ident.load(&(path.to_owned() + "_" + #overwritten_name), loader),
                        },
                        None => quote! {
                            #ident: self.#ident.load(&(path.to_owned() + "_" + stringify!(#ident)), loader),
                        }
                    }
                });
                quote! {
                    Self { #(
                        #fields
                    )* }
                }
            }
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let i = syn::Index::from(i);
                    let i = &i;
                    if is_skip(f) {
                        return quote! { self.#i, };
                    }
                    match find_field_renaming(f) {
                        Some(overwritten_name) => quote! {
                            self.#i.load(&(path.to_owned() + "_" + #overwritten_name), loader),
                        },
                        None => quote! {
                            self.#i.load(&(path.to_owned() + "_" + &#i.to_string()), loader),
                        },
                    }
                });
                quote! {
                    Self ( #(
                        #fields
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
            let enums1 = data.variants.iter().map(|f| {
                find_variant_renaming(f)
                    .map(|overwritten_name| Ident::new(&overwritten_name, f.ident.span()))
                    .unwrap_or_else(|| f.ident.clone())
            });
            let enums2 = data.variants.iter().map(|f| &f.ident);

            quote! {
                match String::default().load(path, loader).as_ref() {
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

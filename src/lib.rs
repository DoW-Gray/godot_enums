use proc_macro::TokenStream;
use quote::quote;
use syn;

extern crate num;

#[proc_macro_derive(GodotConvert)]
pub fn godot_convert_derive(input: TokenStream) -> TokenStream {
    let name = syn::parse::<syn::DeriveInput>(input).unwrap().ident;
    let gen = quote! {
        impl GodotConvert for #name {
            type Via = i32;
        }
    };
    gen.into()
}

#[proc_macro_derive(ToGodot)]
pub fn to_godot_derive(input: TokenStream) -> TokenStream {
    let name = syn::parse::<syn::DeriveInput>(input).unwrap().ident;
    let gen = quote! {
        impl ToGodot for #name {
            fn to_godot(&self) -> i32 { *self as i32 }
        }
    };
    gen.into()
}

#[proc_macro_derive(FromGodot)]
pub fn from_godot_derive(input: TokenStream) -> TokenStream {
    let name = syn::parse::<syn::DeriveInput>(input).unwrap().ident;
    let gen = quote! {
        impl FromGodot for #name {
            fn try_from_godot(via: i32) -> Result<#name, ConvertError> {
                match num::FromPrimitive::from_i32(via) {
                    Some(val) => Ok(val),
                    None => Err(ConvertError::new(
                        format!("Unable to convert {} to {}", via, stringify!(#name))
                    )),
                }
            }
        }
    };
    gen.into()
}

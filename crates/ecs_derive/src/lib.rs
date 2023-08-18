use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    ItemStruct,
};

#[proc_macro_derive(ComponentTrait)]
pub fn derive_component_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    impl_component_trait(&input)
}

fn impl_component_trait(input: &ItemStruct) -> TokenStream {
    let name = &input.ident;

    quote! {
        impl ComponentTrait for #name {
            fn name(&self) -> String { String::from(stringify!(#name)) }
            fn as_any(&mut self) -> &mut dyn std::any::Any { self }
        }
    }.into()
}
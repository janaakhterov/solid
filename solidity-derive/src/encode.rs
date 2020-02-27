use proc_macro2::TokenStream;
use syn::{
    Data,
    DeriveInput,
    Fields,
};

pub(super) fn impl_encode(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    for attr in &ast.attrs {
        match attr.style {
            syn::AttrStyle::Outer => eprintln!("{}", quote! { "AttrStyle: Outer" }),
            syn::AttrStyle::Inner(_) => eprintln!("{}", quote! { "AttrStyle: Inner" }),
        }
    }

    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => todo!(),
        },

        _ => todo!(),
    };

    let count = fields.iter().count();

    let field = fields.iter().map(|field| field.ident.clone());

    let ty = fields.iter().map(|field| field.ty.clone());

    let ty2 = fields.iter().map(|field| field.ty.clone());

    let encode = quote! {
        fn encode(&self) -> Vec<u8> {
            let len = self.required_len();

            let mut buf = vec![0u8; len as usize];

            let mut offset = (#count * 32) as usize;
            let mut index = 0usize;

            // if #has_name {
            //     let mut selector = solidity::Selector::new();
            //     #(
            //         selector = selector.push::<#ty2>();
            //     )*

            //     selector.build(stringify!(#ident));
            // }

            #(
                let bytes = self.#field.encode();
                if <#ty as Encode>::is_dynamic() {
                    buf[index * 32..(index + 1) * 32].copy_from_slice(&(offset as u64).encode());
                    buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                    offset += bytes.len();
                } else {
                    buf[index * 32..(index + 1) * 32].copy_from_slice(&bytes);
                }
                index += 1;
            )*

            buf
        }
    };

    let field = fields.iter().map(|field| field.ident.clone());

    let ty = fields.iter().map(|field| field.ty.clone());

    let required_len = quote! {
        fn required_len(&self) -> u64 {
            let mut len = 0u64;

            #(
                len += if <#ty as Encode>::is_dynamic() {
                    32 + self.#field.required_len()
                } else {
                    32
                };
            )*

            len
        }
    };

    let is_dynamic = quote! {
        fn is_dynamic() -> bool {
            true
        }
    };

    // let header = if let Some(lifetimes) = &ast.lifetimes {
    //     quote! { impl<#lifetimes> Encode for #ident }
    // } else {
    //     quote! { impl Encode for #ident }
    // };

    quote! {

    impl #impl_generics Encode for #ident #ty_generics #where_clause {
            #encode

            #required_len

            #is_dynamic
        }
    }
}

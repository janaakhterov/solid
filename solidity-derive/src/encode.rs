use super::ItemStruct;
use proc_macro2::TokenStream;

pub(super) fn impl_encode(ast: &ItemStruct) -> TokenStream {
    let ident = &ast.ident;

    let count = ast.fields.iter().count();

    let field = ast.fields.iter().map(|field| field.ident.clone());

    let ty = ast.fields.iter().map(|field| field.ty.clone());

    let encode = quote! {
        fn encode(self) -> Vec<u8> {
            let len = self.required_len();

            let mut buf = vec![0u8; len as usize];

            let mut offset = (#count * 32) as usize;
            let mut index = 0usize;

            #(
                let bytes = self.#field.encode();
                if #ty::is_dynamic() {
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

    let field = ast.fields.iter().map(|field| field.ident.clone());

    let ty = ast.fields.iter().map(|field| field.ty.clone());

    let required_len = quote! {
        fn required_len(&self) -> u64 {
            let mut len = 0u64;

            #(
                len += if #ty::is_dynamic() {
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

    let header = if let Some(lifetimes) = &ast.lifetimes {
        quote! { impl<#lifetimes> Encode for #ident }
    } else {
        quote! { impl Encode for #ident }
    };

    quote! {
        #header {
            #encode

            #required_len

            #is_dynamic
        }
    }
}

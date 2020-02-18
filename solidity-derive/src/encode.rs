use super::ItemStruct;
use proc_macro2::TokenStream;

pub(super) fn impl_encode(ast: ItemStruct) -> TokenStream {
    let ident = &ast.ident;

    let count = ast.fields.iter().filter_map(|field| {
        field.ident.as_ref()
    }).count();

    let field = ast.fields.iter().filter_map(|field| {
        field.ident.as_ref()
    });

    let ty = ast.fields.iter().filter_map(|field| {
        if let Some(_) = field.ident {
            Some(field.ty.clone())
        } else {
            None
        }
    });

    let encode = quote! {
        fn encode(self) -> Vec<u8> {
            let len = self.required_len();

            let mut buf = vec![0u8; len];

            let mut offset = #count * 32;
            let mut index = 0;

            #(
                let bytes = self.#field.encode();
                if #ty::is_dynamic() {
                    buf[index * 32..(index + 1) * 32].copy_from_slice(&(offset as u64).encode());
                    buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                    offset += bytes.len();
                } else {
                    buf.copy_from_slice(*bytes);
                }
                index += 1;
            )*

            buf
        }
    };

    let field = ast.fields.iter().filter_map(|field| {
        field.ident.as_ref()
    });

    let ty = ast.fields.iter().filter_map(|field| {
        if let Some(_) = field.ident {
            Some(field.ty.clone())
        } else {
            None
        }
    });

    let required_len = quote ! {
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

    quote! {
        impl solidity_core::encode::Encode for #ident {
            #encode

            #required_len

            #is_dynamic
        }
    }
}

use super::ItemStruct;
use proc_macro2::TokenStream;

pub(super) fn impl_decode(ast: ItemStruct) -> TokenStream {
    let ident = &ast.ident;

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

    quote! {
        impl<'a> solidity_core::decode::Decode<'a> for #ident {
            fn decode(buf: &'a [u8]) -> Self {
                use solidity_core::encode::Encode;
                let mut index = 0;

                Self {
                    #(
                        #field: {
                            let value = if #ty::is_dynamic() {
                                let offset = u64::decode(&buf[32 + index * 32..32 + (index + 1) * 32]) as usize;
                                #ty::decode(&buf[offset..])
                            } else {
                                #ty::decode(&buf[32 + index * 32..32 + (index + 1) * 32])
                            };
                            index += 1;
                            value
                        },
                    )*
                }
            }
        }
    }
}

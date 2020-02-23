use proc_macro2::TokenStream;
use syn::{
    Data,
    DeriveInput,
    Fields,
    FieldsNamed,
    FieldsUnnamed,
};

pub(super) fn impl_decode(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    match &ast.data {
        Data::Struct(ref data) => {
            let fields = match &data.fields {
                Fields::Named(FieldsNamed { named, .. }) => named,
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
                Fields::Unit => unimplemented!(),
            };

            let field = fields.iter().filter_map(|field| field.ident.as_ref());

            let ty = fields.iter().filter_map(|field| {
                if let Some(_) = field.ident {
                    Some(field.ty.clone())
                } else {
                    None
                }
            });

            quote! {
                impl<'a> Decode<'a> for #ident {
                    fn decode(buf: &'a [u8]) -> Self {
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

        _ => unimplemented!(),
    }
}

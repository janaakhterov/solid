use super::Solidity;
use proc_macro2::{
    Span,
    TokenStream,
};
use syn::{
    punctuated::Punctuated,
    token::Add,
    Data,
    DeriveInput,
    Fields,
    GenericParam,
    Lifetime,
    LifetimeDef,
};

pub(super) fn impl_decode(ast: &mut DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    let mut lifetime = LifetimeDef::new(Lifetime::new("'solidity", Span::call_site()));
    let mut bounds = Punctuated::<Lifetime, Add>::new();

    for param in ast.generics.lifetimes() {
        bounds.push(param.lifetime.clone());
    }

    lifetime.bounds = bounds;

    let mut generics = ast.generics.clone();
    generics.params.push(GenericParam::Lifetime(lifetime));

    let (_, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => return quote! {},
        },

        _ => panic!("Solidity does not support enums are unsupported"),
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
        impl #generics Decode<'solidity> for #ident #ty_generics #where_clause {
            fn decode(buf: &'solidity [u8]) -> Self {
                // Solidity returns the function signature for "Error(string)" if a function throws an error.
                // To get around this simply check if the buffer is a factor of 32 or not. This is valid since
                // solidity standard encoding format enforces that all fields line up to a 32 byte boundry.
                let buf = if buf.len() % 32 == 4 {
                    &buf[4..]
                } else {
                    &buf
                };

                let mut index = 0;

                Self {
                    #(
                        #field: {
                            let value = if <#ty as Encode>::is_dynamic() {
                                let offset = u64::decode(&buf[index * 32..(index + 1) * 32]) as usize;
                                <#ty as Decode>::decode(&buf[offset..])
                            } else {
                                <#ty as Decode>::decode(&buf[index * 32..(index + 1) * 32])
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

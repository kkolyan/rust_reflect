extern crate proc_macro;
extern crate core;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use syn::{parse_macro_input, DeriveInput, Data, Fields, Field};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Reflected)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);
    match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => generate_struct(input.ident, &fields.named),
            Fields::Unnamed(_) => panic!(""),
            Fields::Unit => panic!(""),
        },
        Data::Enum(_) => panic!("enums are not supported"),
        Data::Union(_) => panic!("unions are not supported"),
    }
}

fn generate_struct(name: Ident, s: &Punctuated<Field, Comma>) -> TokenStream {
    let fname = s.iter().map(|it| &it.ident).collect::<Vec<_>>();
    let ftype = s.iter().map(|it| &it.ty).collect::<Vec<_>>();

    let tokens = quote! {
        impl Reflected for #name {
            fn create_meta() -> Struct {
                Struct {
                    name: stringify!(#name),
                    type_id: TypeId::of::<#name>().type_id(),
                    fields: HashMap::from([
                        #(
                            Field {
                                name: stringify!(#fname),
                                type_id: TypeId::of::<#ftype>().type_id(),
                                get_ref_delegate: |instance| match instance.downcast_ref::<Self>() {
                                    None => Err(GetError::InvalidTarget),
                                    Some(instance) => Ok(&instance.#fname)
                                },
                                set_delegate: |instance, value| {
                                    match instance.downcast_mut::<Self>() {
                                        None => Err(SetError::InvalidTarget),
                                        Some(instance) => {
                                            match value.downcast_ref::<#ftype>() {
                                                None => Err(SetError::InvalidValueType),
                                                Some(value) => {
                                                    instance.#fname = *value;
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                },
                            }
                        ),*
                    ].map(|it| (it.name, it))),
                    constructor: |mut values| {
                        let mut field_errors = vec![];
                        #(
                            let #fname = match values.remove(stringify!(#fname)) {
                                None => {
                                    field_errors.push(ConstructorFieldError {
                                        field: stringify!(#fname),
                                        resolution: ConstructorFieldErrorResolution::MissingField,
                                    });
                                    None
                                }
                                Some(value) => {
                                    match value.downcast::<#ftype>() {
                                        Ok(value) => {
                                            Some(*value)
                                        }
                                        Err(_) => {
                                            field_errors.push(ConstructorFieldError {
                                                field: stringify!(#fname),
                                                resolution: ConstructorFieldErrorResolution::InvalidType,
                                            });
                                            None
                                        }
                                    }
                                }
                            };
                        )*
                        if field_errors.is_empty() {
                            Ok(Box::from(#name {
                                #(
                                    #fname: #fname.unwrap()
                                ),*
                            }))
                        } else {
                            Err(ConstructorError { field_errors })
                        }
                    },
                }
            }
        }
    };
    tokens.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}

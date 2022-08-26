//! A macro that implements a builder for a struct.
//!
//! This was designed to be used by `eiga`, and thus makes a few assumptions:
//! - The target struct has named fields
//! - Optional fields have their type written as `Option<...>`. The macro won't
//! recognize the `Option` type in any other form, e.g., `std::option::Option`.
//!
//! # Example
//!
//! Applying #[derive(Builder)] to
//!
//! ```
//! # use eiga_builder_derive::Builder;
//!
//! #[derive(Builder)]
//! struct Foo<'a> {
//!     x: i32,
//!     y: Option<&'a str>,
//! }
//! ```
//!
//! generates
//!
//! ```
//! # struct Foo<'a> {
//! #     x: i32,
//! #     y: Option<&'a str>,
//! # }
//!
//! /// A builder for `Foo`.
//! struct FooBuilder<'a> {
//!     x: i32,
//!     y: Option<&'a str>,
//! }
//!
//! impl<'a> FooBuilder<'a> {
//!     fn new(x: i32) -> Self {
//!         Self {
//!             x,
//!             y: None,
//!         }
//!     }
//!
//!     /// Builds a new `Foo` based on the current configuration.
//!     pub fn build(&self) -> Foo<'a> {
//!         Foo {
//!             x: self.x,
//!             y: self.y
//!         }
//!     }
//!
//!     /// Sets the y query string parameter.
//!     pub fn y(&mut self, y: &'a str) -> &mut Self {
//!         self.y = Some(y);
//!
//!         self
//!     }
//! }
//!
//! impl<'a> Foo<'a> {
//!     /// Constructs a new `FooBuilder`.
//!     pub fn builder(x: i32) -> FooBuilder<'a> {
//!         FooBuilder::new(x)
//!     }
//! }
//! ```

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument,
    Generics, PathArguments, Type,
};

#[doc(hidden)]
#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let builder_ident = &format_ident!("{}Builder", ident);
    let generics = &input.generics;
    let struct_data = &input.data;

    let builder_struct =
        builder_struct(ident, builder_ident, generics, struct_data);
    let builder_impl =
        builder_impl(ident, builder_ident, generics, struct_data);
    let builder_method =
        builder_method(ident, builder_ident, generics, struct_data);

    let tokens = quote! {
        #builder_struct
        #builder_impl
        #builder_method
    };

    tokens.into()
}

/// Returns a token stream of the builder struct definition.
fn builder_struct(
    ident: &Ident,
    builder_ident: &Ident,
    generics: &Generics,
    struct_data: &Data,
) -> TokenStream {
    let comment = format!("A builder for `{}`", ident);

    let builder_fields_iter = fields_iter(struct_data).map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;

        quote! { #ident: #ty }
    });
    let builder_fields = quote! { #(#builder_fields_iter),* };

    quote! {
        #[doc = #comment]
        pub struct #builder_ident #generics {
            #builder_fields
        }
    }
}

/// Returns a token stream of the builder struct implementation.
fn builder_impl(
    ident: &Ident,
    builder_ident: &Ident,
    generics: &Generics,
    struct_data: &Data,
) -> TokenStream {
    let new_method = new_method(struct_data);
    let build_method = build_method(ident, generics, struct_data);
    let setters = setters(struct_data);

    quote! {
        impl #generics #builder_ident #generics {
            #new_method
            #build_method
            #setters
        }
    }
}

/// Returns a token stream of the builder's `new` associated method.
fn new_method(struct_data: &Data) -> TokenStream {
    let required_parameters_iter =
        required_fields_iter(struct_data).map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;

            quote! { #ident: #ty }
        });
    let required_parameters = quote! { #(#required_parameters_iter),* };

    let fields_iter = fields_iter(struct_data).map(|field| {
        let ident = &field.ident;

        if is_option(&field.ty) {
            quote! { #ident: None }
        } else {
            quote! { #ident }
        }
    });
    let fields = quote! { #(#fields_iter),* };

    quote! {
        fn new(#required_parameters) -> Self {
            Self {
                #fields
            }
        }
    }
}

/// Returns a token stream of the builder's `build` method.
fn build_method(
    ident: &Ident,
    generics: &Generics,
    struct_data: &Data,
) -> TokenStream {
    let comment = format!(
        "Builds a new `{}` based on the current configuration.",
        ident
    );

    let fields_iter = fields_iter(struct_data).map(|field| {
        let ident = &field.ident;

        quote! { #ident: self.#ident }
    });
    let fields = quote! { #(#fields_iter),* };

    quote! {
        #[doc = #comment]
        pub fn build(&self) -> #ident #generics {
            #ident {
                #fields
            }
        }
    }
}

/// Returns a token stream of the builder's setter methods.
///
/// There's a setter for each optional field on the original struct.
fn setters(data: &Data) -> TokenStream {
    let setters_iter = optional_fields_iter(data).map(|field| {
        // It's safe to unwrap here since optional_fields_iter iterates over
        // named fields only.
        let ident = field.ident.as_ref().unwrap();
        let comment = format!("Sets the {} query string parameter.", ident);
        let ty = inner_type(&field.ty);

        quote! {
            #[doc = #comment]
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);

                self
            }
        }
    });

    quote! { #(#setters_iter)* }
}

/// Returns a token stream of the original struct's `builder` method.
fn builder_method(
    ident: &Ident,
    builder_ident: &Ident,
    generics: &Generics,
    struct_data: &Data,
) -> TokenStream {
    let comment = format!("Constructs a new `{}`.", builder_ident);

    let parameters_iter = required_fields_iter(struct_data).map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;

        quote! { #ident: #ty }
    });
    let parameters = quote! { #(#parameters_iter),* };

    let arguments_iter = required_fields_iter(struct_data).map(|field| {
        let ident = &field.ident;

        quote! { #ident }
    });
    let arguments = quote! { #(#arguments_iter),* };

    quote! {
        impl #generics #ident #generics {
            #[doc = #comment]
            pub fn builder(#parameters) -> #builder_ident #generics {
                #builder_ident::new(#arguments)
            }
        }
    }
}

/// Returns the first generic type argument of a type.
///
/// This function was written for extracting the inner type from an `Option`.
/// I wouldn't expect it work for other use cases.
///
/// # Panics
///
/// Panics if `ty` is not a type path or a generic type.
fn inner_type(ty: &Type) -> &Type {
    let first_type_path_segment = match ty {
        Type::Path(type_path) => match type_path.path.segments.first() {
            Some(path_segment) => path_segment,
            None => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    match &first_type_path_segment.arguments {
        PathArguments::AngleBracketed(arguments) => {
            match arguments.args.first() {
                Some(GenericArgument::Type(ty)) => ty,
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

/// Returns true if `ty` is an `Option`.
///
/// This function returns true if and only if the type is literally written as
/// `Option<...>`. This approach is brittle, but it works for `eiga`.
fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => match type_path.path.segments.first() {
            Some(segment) => segment.ident == "Option",
            None => false,
        },
        _ => false,
    }
}

/// Returns an iterator over the named fields of a struct.
///
/// # Panics
///
/// Panics if `struct_data` doesn't represent a struct with named fields.
fn fields_iter(struct_data: &Data) -> impl Iterator<Item = &Field> {
    match struct_data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

/// Returns an iterator over the optional fields of a struct.
///
/// An optional field is a field of type `Option`.
fn optional_fields_iter(struct_data: &Data) -> impl Iterator<Item = &Field> {
    fields_iter(struct_data).filter(|field| is_option(&field.ty))
}

/// Returns an iterator over the required fields of a struct.
///
/// A required field is a field whose type is not `Option`.
fn required_fields_iter(struct_data: &Data) -> impl Iterator<Item = &Field> {
    fields_iter(struct_data).filter(|field| !is_option(&field.ty))
}

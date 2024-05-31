use proc_macro::TokenStream;
use syn::*;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser, Result};

// Copy of syn's helper stuff for parse_macro_input!
pub fn parse <T: ParseMacroInput> (token_stream: TokenStream) -> Result<T> {
    T::parse.parse(token_stream)
}

pub trait ParseMacroInput: Sized {
    fn parse(input: ParseStream) -> Result <Self>;
}

impl <T: Parse> ParseMacroInput for T {
    fn parse(input: ParseStream) -> Result <Self> {
        <T as Parse>::parse(input)
    }
}

pub fn add_add_to_span_to_enum(ty: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = ty.generics.split_for_impl();
	let name = &ty.ident;
	let mut arms: Vec <Arm> = vec![];
	for variant in &ty.variants {
		let vname = &variant.ident;
		arms.push(parse_quote!(#name::#vname(x) => x.add_to_span(offset)))
	}

	quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			fn __add_to_span(&mut self, offset: usize) {
				match self {
					#(#arms),*
				}
			}
		}
	}
}

pub fn add_trait_bounds(generics: &mut Generics, type_param_bounds: impl Iterator <Item = TypeParamBound> + Clone) {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.extend(type_param_bounds.clone());
        }
    }
}

pub fn mk_parseable_generics(generics: &mut Generics) {
	let bound = [parse_quote::parse::<TypeParamBound>(quote!(Parseable))];
	add_trait_bounds(generics, bound.into_iter())
}

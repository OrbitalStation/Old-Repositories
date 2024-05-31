use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::*;

pub fn spanned(attrs: TokenStream, item: TokenStream) -> TokenStream {
	match crate::parse::<ItemStruct>(item.clone()) {
		Ok(x) => process_struct(x),
		Err(_) => match crate::parse::<ItemEnum>(item.clone()) {
			Ok(x) => process_enum(x),
			Err(_) => process_impl(attrs, parse_macro_input!(item))
		}
	}
}

fn process_enum(mut ty: ItemEnum) -> TokenStream {
	crate::pmi::mk_parseable_generics(&mut ty.generics);
	let ts = crate::pmi::add_add_to_span_to_enum(&ty);
	let ty = ty.to_token_stream();
	quote!(#ty #ts).into()
}

fn process_impl(attrs: TokenStream, mut imp: ItemImpl) -> TokenStream {
	let attrs = attrs.to_string();
	if attrs.is_empty() {
		imp.items.push(ImplItem::Method(parse_quote! {
			fn add_to_span(&mut self, offset: usize) {
				self.span.val.add(offset)
			}
		}));
	} else if attrs == "enum" {
		imp.items.push(ImplItem::Method(parse_quote! {
			fn add_to_span(&mut self, offset: usize) {
				self.__add_to_span(offset)
			}
		}));
	} else {
		panic!("unknown attrs")
	}

	imp.to_token_stream().into()
}

fn process_struct(mut ty: ItemStruct) -> TokenStream {
	match &mut ty.fields {
		Fields::Named(named) => named.named.push(Field {
			attrs: vec![],
			vis: Visibility::Public(VisPublic {
				pub_token: token::Pub::default()
			}),
			ident: Some(Ident::new("span", Span::call_site())),
			colon_token: Some(token::Colon::default()),
			ty: parse_quote!(Leaf <Span>),
		}),
		_ => panic!("expected named fields")
	}

	ty.to_token_stream().into()
}

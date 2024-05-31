use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;
use syn::punctuated::Pair;

pub fn derive_parseable(item: TokenStream) -> TokenStream {
	match crate::parse::<ItemStruct>(item.clone()) {
		Ok(x) => process_struct(x),
		Err(_) => process_enum(parse_macro_input!(item))
	}
}

fn process_enum(mut ty: ItemEnum) -> TokenStream {
	crate::pmi::mk_parseable_generics(&mut ty.generics);
	let add_to_span = crate::pmi::add_add_to_span_to_enum(&ty);
	let ty_name = &ty.ident;

	// Get rid of the trailing comma(if any)
	if ty.variants.trailing_punct() {
		let Pair::Punctuated(v, _) = ty.variants.pop().unwrap() else { unreachable!() };
		ty.variants.push_value(v)
	}

	let (either_ty, expr) = {
		let mut either_type = String::new();
		let mut expr = String::new();

		for variant in ty.variants.pairs() {
			let field_ty = match &variant.value().fields {
				Fields::Unnamed(unnamed) => unnamed.unnamed.first().unwrap().ty.to_token_stream().to_string(),
				_ => panic!("expected unnamed fields")
			};

			match variant {
				Pair::Punctuated(variant, _) => {
					either_type.push_str(&format!("ParseEither::<{field_ty},"));
					expr.push_str(&format!("match x {{ ParseEither::A(x) => Self::{}(x), ParseEither::B(x) => ", variant.ident))
				},
				Pair::End(variant) => {
					either_type.push_str(&field_ty);
					expr.push_str(&format!("Self::{}(x)", variant.ident))
				}
			}
		}
		either_type.push_str(&">".repeat(ty.variants.len() - 1));
		expr.push_str(&"}".repeat(ty.variants.len() - 1));

		let either_ty = crate::parse::<Type>(either_type.parse().unwrap()).unwrap();
		let expr = crate::parse::<Expr>(expr.parse().unwrap()).unwrap();

		(either_ty, expr)
	};

	let (impl_generics, ty_generics, where_clause) = ty.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics Parseable for #ty_name #ty_generics #where_clause {
			fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
				parser.parse::<#either_ty>().map(|x| #expr)
			}

			fn add_to_span(&mut self, offset: usize) {
				self.__add_to_span(offset)
			}
		}
	};

	quote!(#add_to_span #expanded).into()
}

fn process_struct(mut ty: ItemStruct) -> TokenStream {
	let ty_name = ty.ident;

	let fields = match ty.fields {
		Fields::Named(named) => named.named,
		_ => panic!("expected named fields")
	};

	let mut stmts: Vec <Stmt> = vec![];
	let mut init: Vec <Ident> = vec![];
	let mut add_to_span: Vec <Stmt> = vec![];
	let mut isnt_first = false;
	let mut prev_was_whitespace = false;

	for field in fields.into_pairs() {
		let field = field.into_value();
		let field_name = field.ident.unwrap();

		let ty = field.ty.to_token_stream().to_string();

		let dont_place_whitespace = ty == "Newline" || ty == "Whitespace" || ty.starts_with("Block") || prev_was_whitespace;

		// A small optimization
		if isnt_first && !dont_place_whitespace {
			stmts.push(parse_quote!(parser.whitespace()?;))
		} else {
			isnt_first = true
		}

		prev_was_whitespace = ty == "Whitespace";

		stmts.push(parse_quote!(let #field_name = parser.parse()?;));
		add_to_span.push(parse_quote!(self.#field_name.add_to_span(offset);));
		init.push(field_name);
	}

	crate::pmi::mk_parseable_generics(&mut ty.generics);
	let (impl_generics, ty_generics, where_clause) = ty.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics Parseable for #ty_name #ty_generics #where_clause {
			fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
				#(#stmts)*
				Ok(Self {
					#(#init),*
				})
			}

			fn add_to_span(&mut self, offset: usize) {
				#(#add_to_span)*
			}
		}
	};

	expanded.into()
}

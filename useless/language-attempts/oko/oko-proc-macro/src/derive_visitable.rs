use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;
use syn::__private::TokenStream2;

pub fn derive_visitable(item: TokenStream) -> TokenStream {
	match crate::parse::<ItemStruct>(item.clone()) {
		Ok(x) => process_struct(x),
		Err(_) => process_enum(parse_macro_input!(item))
	}
}

fn process_enum(ty: ItemEnum) -> TokenStream {
	let mut params = vec![];
	let arms = ty.variants.into_iter().filter_map::<Arm, _>(|variant| {
		if variant.attrs.iter().find(|x| {
			let mut x = x.to_token_stream().to_string();
			x.retain(|x| !x.is_whitespace());
			x == "#[novisit]"
		}).is_some() {
			return None
		}
		add_to_params_if_bounds_on_a_field(&mut params, match variant.fields {
			Fields::Unnamed(u) => u.unnamed.into_iter().next().unwrap().ty,
			_ => panic!("expected unnamed associated data")
		}, &ty.generics);
		let variant = variant.ident;
		Some(parse_quote!(Self::#variant(x) => x.visit_mut(fun, cfg),))
	});

	let shenanigans = quote!(match self { #(#arms)* });
	generalized_process(ty.ident, ty.generics, shenanigans, params)
}

fn process_struct(ty: ItemStruct) -> TokenStream {
	let mut params = vec![];
	let stmts = ty.fields.into_iter().filter_map::<Stmt, _>(|field| {
		if field.attrs.iter().find(|x| {
			let mut x = x.to_token_stream().to_string();
			x.retain(|x| !x.is_whitespace());
			x == "#[novisit]"
		}).is_some() {
			return None
		}
		add_to_params_if_bounds_on_a_field(&mut params, field.ty, &ty.generics);
		let field: Expr = if let Some(x) = field.ident { parse_quote!(#x) } else { parse_quote!(0) };
		Some(parse_quote!(self.#field.visit_mut(fun, cfg);))
	});

	let shenanigans = quote!(#(#stmts)*);
	generalized_process(ty.ident, ty.generics, shenanigans, params)
}

fn add_to_params_if_bounds_on_a_field(params: &mut Vec <Ident>, ty: Type, generics: &Generics) {
	match ty {
		Type::Path(path) => for segment in path.path.segments {
			match segment.arguments {
				PathArguments::AngleBracketed(b) => for generic in b.args {
					match generic {
						GenericArgument::Type(t) => {
							let ident = match crate::parse::<Ident>(t.to_token_stream().into()) {
								Err(_) => continue,
								Ok(x) => x
							};
							for gen in &generics.params {
								match gen {
									GenericParam::Type(ty) if ty.ident == ident => {
										params.push(ident);
										break
									},
									_ => ()
								}
							}
						},
						_ => ()
					}
				},
				_ => ()
			}
		},
		_ => ()
	}
}

fn generalized_process(name: Ident, mut generics: Generics, mut shenanigans: TokenStream2, params_to_generalize: Vec <Ident>) -> TokenStream {
	for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
			if params_to_generalize.contains(&type_param.ident) {
				type_param.bounds.push(parse_quote::parse::<TypeParamBound>(quote!(Visitable)))
			}
        }
    }

	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// The only scope-increasing type
	if name.to_string().starts_with("Block") {
		shenanigans = quote! {
			if !cfg.scope_limited {
				#shenanigans
			}
		}
	}

	let expanded = quote! {
		impl #impl_generics Visitable for #name #ty_generics #where_clause {
			fn visit_mut <U> (&mut self, fun: &mut impl VisitCallback <U>, cfg: &VisitProperties) {
				visit_self(self, fun);
				#shenanigans
			}
		}
	};

	expanded.into()
}

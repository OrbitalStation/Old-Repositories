use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;
use syn::punctuated::Punctuated;

///
/// Adds a constructor method `new`
///
pub fn constructor(_attrs: TokenStream, item: TokenStream) -> TokenStream {
	let ty = syn::parse_macro_input!(item as syn::ItemStruct);

	let ty_name = ty.ident.clone();
	let ty_args =  ty.fields.iter().map(|field| FnArg::Typed(PatType {
		attrs: vec![],
		pat: Box::new(Pat::Path(PatPath {
			attrs: vec![],
			qself: None,
			path: Path {
				leading_colon: None,
				segments: Punctuated::from_iter([PathSegment {
					ident: field.ident.as_ref().unwrap().clone(),
					arguments: PathArguments::None
				}]),
			},
		})),
		colon_token: Default::default(),
		ty: Box::new(field.ty.clone()),
	}));
	let ty_init = ty_args.clone().zip(&ty.fields).map(|(arg, field)| match arg {
		FnArg::Typed(mut typed) => {
			typed.ty = Box::new(Type::Path(TypePath {
				qself: None,
				path: Path {
					leading_colon: None,
					segments: Punctuated::from_iter([PathSegment {
						ident: field.ident.as_ref().unwrap().clone(),
						arguments: PathArguments::None,
					}]),
				}
			}));
			typed
		},
		_ => unreachable!()
	});

	let mut ts = ty.to_token_stream();
	ts.extend(quote! {
		impl #ty_name {
			pub fn new(#(#ty_args),*) -> Self {
				Self {
					#(#ty_init),*
				}
			}
		}
	});
	ts.into()
}

use proc_macro::TokenStream;

mod derive_parseable;
mod derive_visitable;
mod spanned;
mod constructor;
mod pmi;

use pmi::*;

#[proc_macro_derive(Parseable)]
pub fn derive_parseable(item: TokenStream) -> TokenStream {
	derive_parseable::derive_parseable(item)
}

#[proc_macro_derive(Visitable, attributes(novisit))]
pub fn derive_visitable(item: TokenStream) -> TokenStream {
	derive_visitable::derive_visitable(item)
}

#[proc_macro_attribute]
pub fn spanned(attrs: TokenStream, item: TokenStream) -> TokenStream {
	spanned::spanned(attrs, item)
}

#[proc_macro_attribute]
pub fn constructor(attrs: TokenStream, item: TokenStream) -> TokenStream {
	constructor::constructor(attrs, item)
}

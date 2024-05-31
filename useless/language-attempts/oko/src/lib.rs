#![allow(incomplete_features)]

#![feature(adt_const_params)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(result_option_inspect)]

extern crate oko_proc_macro;
pub use oko_proc_macro::*;

#[macro_export]
macro_rules! modules {
    ($( $i:ident )*) => {$(
		mod $i;
		pub use $i::*;
	)*};
}

modules!(utils ast);

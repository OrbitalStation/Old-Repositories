use proc_macro::TokenStream;

#[proc_macro]
pub fn concat(args: TokenStream) -> TokenStream {
    let mut result = String::new();
    for concat in args.to_string().split("~") {
        result.push_str(concat.trim())
    }
    result.parse().unwrap()
}

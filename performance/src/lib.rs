use quote::{quote};
use syn::*;
use syn::__private::TokenStream;

#[proc_macro_attribute]
pub fn calculate_delay(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let function_name = &input_fn.sig.ident;

    let function_body = &input_fn.block;

    // let start = std::time::Instant::now();
    // let elapsed =  start.elapsed().as_millis();
    let result = quote! {
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        #[allow(unused_variables)]
        #[allow(unused_braces)]
        fn #function_name() {
            let start = std::time::Instant::now();
            let result = { #function_body };
            let elapsed =  start.elapsed().as_millis();
            println!("Tiempo de ejecución de {} es {:?} ms.", stringify!(#function_name), elapsed);
            result
        }
    };

    // Devolvemos el TokenStream resultante
    result.into()
}
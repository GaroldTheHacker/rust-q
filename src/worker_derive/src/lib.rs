extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

mod worker_derive {
    pub fn worker_derive(input: TokenStream) -> TokenStream {
        let ast = syn::parse(input).unwrap();

        impl_worker_methods(&ast);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

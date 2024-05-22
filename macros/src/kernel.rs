use proc_macro2::TokenStream;
use quote::quote;

pub fn render() -> TokenStream {
    quote!({
        #[cfg(fw = "1100")]
        let k = okf_1100::Kernel::new();

        k
    })
}

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Error, Path};

pub fn render() -> syn::Result<TokenStream> {
    let ty: Path = if cfg!(fw = "1100") {
        parse_quote!(okf_1100::Kernel)
    } else {
        return Err(Error::new(
            Span::call_site(),
            "invalid `fw` configuration, see okf README for the instructions",
        ));
    };

    Ok(ty.into_token_stream())
}

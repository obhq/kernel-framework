use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemStruct, LitInt, TraitItem};

mod derive;
mod offset;
mod panic_handler;

#[proc_macro_derive(MappedKernel)]
pub fn derive_mapped_kernel(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    self::derive::mapped_kernel(item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn offset(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitInt);
    let item = parse_macro_input!(item as TraitItem);

    self::offset::transform(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn panic_handler(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitInt);

    self::panic_handler::transform(args)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemStruct, LitInt, TraitItem};

mod derive;
mod kernel;
mod offset;

#[proc_macro_derive(MappedKernel)]
pub fn derive_mapped_kernel(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    self::derive::mapped_kernel(item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Renders kernel type based on `fw` configuration.
///
/// This macro was designed to be used by the application. Use trait bound with `okf::Kernel`
/// instead if you are building a library.
#[proc_macro]
pub fn kernel(_: TokenStream) -> TokenStream {
    self::kernel::render()
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

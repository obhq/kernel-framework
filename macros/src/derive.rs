use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Fields, ItemStruct, Type};

pub fn mapped_kernel(item: ItemStruct) -> syn::Result<TokenStream> {
    // Check if single unnamed field.
    let fields = match item.fields {
        Fields::Unnamed(v) if v.unnamed.len() == 1 => v,
        v => {
            return Err(Error::new_spanned(
                v,
                "expect a struct with an unnamed field",
            ));
        }
    };

    // Check field type.
    let ty = match &fields.unnamed[0].ty {
        Type::Reference(v) => v,
        v => return Err(Error::new_spanned(v, "expect a static reference of `[u8]`")),
    };

    if !ty.lifetime.as_ref().is_some_and(|l| l.ident == "static") {
        return Err(Error::new_spanned(
            ty,
            "expect a static reference of `[u8]`",
        ));
    }

    if let Some(m) = ty.mutability {
        return Err(Error::new_spanned(m, "expect a non-mutable reference"));
    }

    // Check type element.
    let elem = match ty.elem.as_ref() {
        Type::Slice(v) => v,
        v => return Err(Error::new_spanned(v, "expect `[u8]`")),
    };

    match elem.elem.as_ref() {
        Type::Path(t) if t.qself.is_none() && t.path.is_ident("u8") => {}
        v => return Err(Error::new_spanned(v, "expect `u8`")),
    }

    // Render MappedKernel implementation.
    let ident = item.ident;

    Ok(quote! {
        impl okf::MappedKernel for #ident {
            unsafe fn new(base: *const u8) -> Self {
                // Get ELF loaded size.
                let e_phnum = base.add(0x38).cast::<u16>().read() as usize;
                let progs = core::slice::from_raw_parts(base.add(0x40), e_phnum * 0x38);
                let mut end = base as usize;

                for h in progs.chunks_exact(0x38) {
                    // Skip anything that is not PT_LOAD and PT_SCE_RELRO.
                    let ty = u32::from_le_bytes(h[0x00..0x04].try_into().unwrap());

                    if !matches!(ty, 0x1 | 0x61000010) {
                        continue;
                    }

                    // Update end address.
                    let addr = usize::from_le_bytes(h[0x10..0x18].try_into().unwrap());
                    let len = usize::from_le_bytes(h[0x28..0x30].try_into().unwrap());
                    let align = usize::from_le_bytes(h[0x30..0x38].try_into().unwrap());

                    assert!(addr >= end); // Just in case if Sony re-order the programs.

                    end = addr + len.next_multiple_of(align);
                }

                // Get mapped ELF.
                let len = end - (base as usize);
                let mapped = core::slice::from_raw_parts(base, len);

                Self(mapped)
            }

            unsafe fn mapped(self) -> &'static [u8] {
                self.0
            }
        }
    })
}

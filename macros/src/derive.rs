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
        Type::Ptr(v) => v,
        v => return Err(Error::new_spanned(v, "expect a pointer")),
    };

    if let Some(t) = ty.mutability {
        return Err(Error::new_spanned(t, "expect `const`"));
    }

    // Check type element.
    match ty.elem.as_ref() {
        Type::Path(p) if p.qself.is_none() && p.path.is_ident("u8") => {}
        v => return Err(Error::new_spanned(v, "expect `u8`")),
    }

    // Render implementations.
    let ident = item.ident;

    Ok(quote! {
        impl #ident {
            /// # Safety
            /// `LSTAR` register must be original value.
            unsafe fn new() -> Self {
                // Read LSTAR register.
                let mut edx = 0u32;
                let mut eax = 0u32;

                core::arch::asm!(
                    "rdmsr",
                    in("ecx") 0xc0000082u32,
                    out("edx") edx,
                    out("eax") eax,
                    options(pure, nomem, preserves_flags, nostack)
                );

                // Get base address of the kernel.
                let aslr = ((edx as usize) << 32 | (eax as usize)) - 0xffffffff822001c0;
                let base = aslr + 0xffffffff82200000;

                Self(base as *const u8)
            }
        }

        impl okf::MappedKernel for #ident {
            fn addr(self) -> *const u8 {
                self.0
            }
        }
    })
}

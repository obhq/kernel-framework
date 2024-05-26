use proc_macro2::TokenStream;
use quote::quote;
use syn::LitInt;

pub fn transform(arg: LitInt) -> syn::Result<TokenStream> {
    let off: usize = arg.base10_parse()?;

    Ok(quote! {
        #[cfg(feature = "panic-handler")]
        struct PanicMsg<const L: usize> {
            buf: [u8; L],
            pos: usize,
        }

        #[cfg(feature = "panic-handler")]
        impl<const L: usize> core::fmt::Write for PanicMsg<L> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                // This method need to be careful not to cause any panic so we don't end up nested panic.
                let str = s.as_bytes();
                let end = core::cmp::min(self.pos.saturating_add(str.len()), self.buf.len() - 1);
                let buf = unsafe { self.buf.get_unchecked_mut(self.pos..end) };

                buf.copy_from_slice(unsafe { str.get_unchecked(..buf.len()) });
                self.pos = end;

                Ok(())
            }
        }

        #[cfg(feature = "panic-handler")]
        #[panic_handler]
        fn panic(i: &core::panic::PanicInfo) -> ! {
            use core::fmt::Write;
            use okf::MappedKernel;

            // Write panic message.
            let mut m = PanicMsg {
                buf: [0; 1024],
                pos: 0,
            };

            write!(m, "{i}").unwrap();

            // Invoke panic function.
            let k = Kernel::default();
            let f = unsafe { k.addr().add(#off) };
            let f: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...) -> ! =
                unsafe { core::mem::transmute(f) };

            unsafe { f(c"%s".as_ptr(), m.buf.as_ptr()) };
        }
    })
}

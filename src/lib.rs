use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{braced, parse_macro_input, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;


struct ImportArgs {
    arch: Ident,
    _arrow: Token![=>],
    _brace: Brace,
    items: Punctuated<Ident, Token![,]>,
}

impl Parse for ImportArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let items;
        Ok(Self {
            arch: input.parse()?,
            _arrow: input.parse()?,
            _brace: braced!(items in input),
            items: items.parse_terminated(Ident::parse)?,
        })
    }
}


#[proc_macro]
pub fn import_offsets(item: TokenStream) -> TokenStream {
    let offsets = parse_macro_input!(item as ImportArgs);
    let mut output = quote!(
        use vex_sys::Int;
    );
    for reg in &offsets.items {
        // switch order of arguments to make the span of the output depend on the register.
        let offset = format_ident!("OFFSET_{1}_{0}", reg, offsets.arch);
        output = quote!(
            #output
            pub const #reg: Int = vex_sys::#offset as Int;
        );
    }
    // Some arches (MIPS) have lowercase register names.
    quote!(
        #[allow(non_upper_case_globals)]
        pub mod offset {
            #output
        }
    ).into()
}

#[proc_macro]
pub fn import_hwcaps(item: TokenStream) -> TokenStream {
    let hwcaps = parse_macro_input!(item as ImportArgs);
    let arch = hwcaps.arch.to_string().to_uppercase();
    let mut output = quote!();
    for hwcap in &hwcaps.items {
        let offset = format_ident!("VEX_HWCAPS_{}_{}", arch, hwcap);
        output = quote!(
            #output
            pub use vex_sys::#offset as #hwcap;
        );
    }
    quote!(
        pub mod hwcap {
            #output
        }
    ).into()
}

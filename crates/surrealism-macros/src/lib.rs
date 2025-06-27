use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, FnArg, ItemFn, Lit, Meta, MetaNameValue, PatType,
    Expr, ExprLit, ReturnType, punctuated::Punctuated, token::Comma,
};

#[proc_macro_attribute]
pub fn surrealism(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Comma>::parse_terminated);
    let input_fn = parse_macro_input!(item as ItemFn);

    let mut is_default = false;
    let mut export_name_override: Option<String> = None;

    for meta in args.iter() {
        match meta {
            Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("name") => {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = value {
                    let val = s.value();
                    if !val.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                        panic!("#[surrealism(name = \"...\")] must use only ASCII letters, digits, and underscores");
                    }
                    export_name_override = Some(val);
                }
            }
            Meta::Path(path) if path.is_ident("default") => {
                is_default = true;
            }
            _ => panic!("Unsupported attribute: expected #[surrealism], #[surrealism(default)], or #[surrealism(name = \"...\")]"),
        }
    }

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    // Collect argument patterns and types
    let mut arg_patterns = Vec::new();
    let mut arg_types = Vec::new();

    for arg in &fn_sig.inputs {
        match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                arg_patterns.push(pat.clone());
                arg_types.push(ty);
            }
            FnArg::Receiver(_) => panic!("`self` is not supported in #[surrealism] functions"),
        }
    }

    // Compose tuple type and pattern (single args are passed directly)
    let (tuple_type, tuple_pattern) = if arg_types.len() == 1 {
        (quote! { (#(#arg_types),*,) }, quote! { (#(#arg_patterns),*,) })
    } else {
        (quote! { ( #(#arg_types),*, ) }, quote! { ( #(#arg_patterns),*, ) })
    };

    // Return type
    let output_type = match &fn_sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    // Export function names
    let export_suffix = if is_default {
        String::new()
    } else {
        export_name_override.unwrap_or_else(|| fn_name.to_string())
    };

    let export_ident = format_ident!("__sr_fnc__{}", export_suffix);
    let args_ident = format_ident!("__sr_args__{}", export_suffix);
    let returns_ident = format_ident!("__sr_returns__{}", export_suffix);

    let expanded = quote! {
        #fn_vis #fn_sig #fn_block

        #[unsafe(no_mangle)]
        pub extern "C" fn #export_ident(ptr: u32, len: u32) -> u32 {
            use surrealism::types::convert::Transfer;
            let mut controller = surrealism::Controller {};
            let f = surrealism::SurrealismFunction::<#tuple_type, #output_type, _>::from(
                |#tuple_pattern: #tuple_type| #fn_name(#(#arg_patterns),*)
            );
            f.invoke_raw(&mut controller, (ptr, len).into())
                .unwrap()
                .ptr
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn #args_ident() -> u32 {
            use surrealism::types::convert::Transfer;
            let mut controller = surrealism::Controller {};
            let f = surrealism::SurrealismFunction::<#tuple_type, #output_type, _>::from(
                |#tuple_pattern: #tuple_type| #fn_name(#(#arg_patterns),*)
            );
            f.args_raw(&mut controller)
                .unwrap()
                .ptr
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn #returns_ident() -> u32 {
            use surrealism::types::convert::Transfer;
            let mut controller = surrealism::Controller {};
            let f = surrealism::SurrealismFunction::<#tuple_type, #output_type, _>::from(
                |#tuple_pattern: #tuple_type| #fn_name(#(#arg_patterns),*)
            );
            f.returns_raw(&mut controller)
                .unwrap()
                .ptr
        }
    };

    TokenStream::from(expanded)
}
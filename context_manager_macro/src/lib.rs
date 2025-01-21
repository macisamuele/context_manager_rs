#![deny(
    dead_code,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    unused_imports,
    unused_variables
)]

//! Implementation of the procedural macros exposed by [`context_manager`](https://crates.io/crates/context-manager) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Block;
use syn::Error;
use syn::ItemFn;
use syn::Type;

struct Args {
    context_type: Type,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.is_empty() {
            Err(Error::new(
                input.span(),
                "Expected a type as argument: `#[wrap(Type)]` or `#[async_wrap(Type)]`",
            ))
        } else {
            Ok(Self {
                context_type: input.parse::<Type>()?,
            })
        }
    }
}

/// Procedural macro that will decorate the incoming function with the provided context.
///
/// The context is expected to be a type that implements the `context_manager::SyncWrapContext` trait.
///
/// More documentation available [here](https://docs.rs/context_manager/latest/context_manager/attr.wrap.html)
#[proc_macro_attribute]
pub fn wrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut in_func = parse_macro_input!(item as ItemFn);

    if in_func.sig.constness.is_some() {
        // Insert compile error at the begin of the function block.
        // Doing so allows a clear compile failure, while allowing type inference to still work.
        in_func.block.stmts.insert(
            0,
            parse_quote!(::std::compile_error!("#[wrap] cannot operate on const functions.");),
        );
        return quote! { #in_func }.into();
    };

    let args: Args = parse_macro_input!(attr);

    let context_type = &args.context_type;
    let block = &in_func.block;
    let new_body: TokenStream = if in_func.sig.asyncness.is_some() {
        quote! {
            {
                <#context_type as ::context_manager::SyncWrapContext<_>>::run_async(async #block).await
            }
        }
        .into()
    } else {
        quote! {
            {
                <#context_type as ::context_manager::SyncWrapContext<_>>::run_sync(move || #block)
            }
        }
        .into()
    };

    in_func.block.stmts = parse_macro_input!(new_body as Block).stmts;

    quote! { #in_func }.into()
}

/// Procedural macro that will decorate the incoming async function with the provided context.
///
/// The context is expected to be a type that implements the `context_manager::AsyncWrapContext` trait.
///
/// More documentation available [here](https://docs.rs/context_manager/latest/context_manager/attr.async_wrap.html)
#[proc_macro_attribute]
pub fn async_wrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut in_func = parse_macro_input!(item as ItemFn);

    if in_func.sig.constness.is_some() {
        // This is not really possible, because "functions cannot be both `const` and `async`"
        // but let's keep this check for future-proofing
        // Insert compile error at the begin of the function block.
        // Doing so allows a clear compile failure, while allowing type inference to still work.
        in_func.block.stmts.insert(
            0,
            parse_quote!(::std::compile_error!("#[wrap] cannot operate on const functions.");),
        );
        return quote! { #in_func }.into();
    };

    let args: Args = parse_macro_input!(attr);

    let context_type = &args.context_type;
    let block = &in_func.block;
    if in_func.sig.asyncness.is_some() {
        let new_body: TokenStream = quote! {
            {
                <#context_type as ::context_manager::AsyncWrapContext<_>>::run(async #block).await
            }
        }
        .into();
        in_func.block.stmts = parse_macro_input!(new_body as Block).stmts;
    } else {
        // Insert compile error at the begin of the function block.
        // Doing so allows a clear compile failure, while allowing type inference to still work.
        in_func.block.stmts.insert(
            0,
            parse_quote!({::std::compile_error!(
                "#[async_wrap] cannot operate on sync functions. Please consider using a #[wrap] macro or converting/wrapping the function to be async."
            )})
        );
    };

    quote! { #in_func }.into()
}

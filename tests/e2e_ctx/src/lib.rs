use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Injects a `TestCtx` lifecycle around an async test function.
///
/// This attribute rewrites the annotated function into an argument-less async
/// function that:
///
/// 1. Creates a new `TestCtx` before the test body runs
/// 2. Executes the original function body inside a panic boundary
/// 3. Always awaits `TestCtx::cleanup()`
/// 4. Re-throws any panic so the test fails normally
///
/// # Requirements
///
/// - The annotated function **must be `async`**
/// - The annotated function **must not take any arguments**
/// - A type named `TestCtx` must be in scope
/// - The crate using this macro **must depend on the `futures` crate**
///
/// # Panic behavior
///
/// Panics inside the test body are caught, cleanup is guaranteed to run,
/// and the panic is re-raised so the test output and backtrace are preserved.
///
/// # Example
///
/// ```rust
/// use futures::FutureExt;
///
/// #[inject]
/// #[tokio::test]
/// async fn homepage() {
///     // test body can freely use `ctx`
///     ctx.client.goto("/").await;
/// }
/// ```
///
/// # Notes
///
/// This macro intentionally does not rely on `Drop` for cleanup, as async
/// destructors are not supported in Rust. Cleanup is deterministic and
/// explicitly awaited.
#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;
    let fn_sig = &input_fn.sig;
    let fn_name = &fn_sig.ident;
    let fn_block = &input_fn.block;

    // NOTE: assumes async function
    // NOTE: assumes function can not have arguments (e.g from other macros)
    let expanded = quote! {
        use futures::FutureExt;

        #(#fn_attrs)*
        #fn_vis async fn #fn_name() {
            let ctx = TestCtx::new().await;
            let result = std::panic::AssertUnwindSafe(async {
                #fn_block
            })
            .catch_unwind()
            .await;

            ctx.cleanup().await;

            if let Err(panic) = result {
                std::panic::resume_unwind(panic);
            };
        }
    };

    TokenStream::from(expanded)
}

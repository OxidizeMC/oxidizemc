use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Block, Ident, ItemFn, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

fn get_config_var(key: &str) -> String {
    // TODO: Include link to usage docs on build script configuration
    std::env::var(key)
        .expect(format!("Config var {:?} is not set; Make sure you are configuring OxidizeMC in your build.rs script", key).as_str())
}

#[proc_macro_attribute]
pub fn entrypoint(initializer: TokenStream, input: TokenStream) -> TokenStream {
    let initializers_env_var: String = get_config_var("OXIDIZEMC_MOD_INITIALIZERS");
    let valid_initializers: Vec<&str> = initializers_env_var.split(";").collect();

    let initializer: Punctuated<Ident, Token![,]> =
        Punctuated::<Ident, Token![,]>::parse_terminated
            .parse(initializer)
            .unwrap();
    assert!(
        initializer.len() == 1,
        "You must specify the initializer to use when using the `entrypoint()` macro"
    );
    let initializer: String = initializer.get(0).unwrap().to_string();
    if !valid_initializers.contains(&initializer.as_str()) {
        panic!(
            "{:?} is not a valid initializer.\nValid initializers: {:?}",
            initializer, valid_initializers
        )
    }

    let input: ItemFn = parse_macro_input!(input as ItemFn);
    let fn_name: &Ident = &input.sig.ident;
    let fn_block: &Box<Block> = &input.block;
    let jni_fn_name: Ident = format_ident!(
        "Java_{}_Natives_{}_1init",
        get_config_var("OXIDIZEMC_JAVA_PACKAGE").replace(".", "_"),
        initializer
    );

    let expanded: proc_macro2::TokenStream = quote! {
        #[unsafe(no_mangle)]
        extern "system" fn #jni_fn_name(
            env: ::oxidizemc::sys::__Env,
            _this: *mut (),
        ) {
            ::oxidizemc::__java_entrypoint(env, #fn_name);
        }

        fn #fn_name() {
            #fn_block
        }
    };

    TokenStream::from(expanded)
}

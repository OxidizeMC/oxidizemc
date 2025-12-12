#[cfg(feature = "derive")]
extern crate oxidizemc_derive as derive;

mod jvm;
pub mod sys {
    pub type __Env<'a> = java_oxide::Env<'a>;
}
#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "derive")]
pub use derive::entrypoint;

/// Global reference to the Global JVM
pub static JVM: jvm::JVM = jvm::JVM::new();

/// Central java entrypoint
#[doc(hidden)]
pub fn __java_entrypoint<F: FnOnce()>(env: java_oxide::Env, user_main: F) {
    JVM.init(env.vm());
    user_main();
}

use java_oxide::{
    VM, Env
};

/// Container for the Global JVM reference
pub struct JVM {
    inner: std::sync::OnceLock<VM>,
}

impl JVM {
    /// Creates a new uninitialized JVM reference
    #[doc(hidden)]
    pub(crate) const fn new() -> Self {
        JVM {
            inner: std::sync::OnceLock::new(),
        }
    }

    /// Initializes the JVM to `jvm`
    ///
    /// # Panics
    ///
    /// Panics if the JVM was previously initialized, and the new JVM is not the same
    #[doc(hidden)]
    pub(crate) fn init(&self, jvm: VM) {
        if let Err(_) = self.inner.set(jvm) && jvm != *self.get() {
            panic!("Attempted to reinitialize the Global JVM reference with a different JVM reference")
        }
    }

    /// Gets a reference to the initialized JVM
    ///
    /// # Panics
    ///
    /// Panics if the JVM was __NOT__ previously initialized
    pub fn get(&self) -> &VM {
        self.inner.get().expect("Attempted to get the JVM reference before it was initialized")
    }

    /// Same as:
    /// ```rust
    /// JVM.get().with_env(|env| {
    ///     // ...
    /// })
    /// ```
    pub fn with_env<F, R>(&self, callback: F) -> R
    where
        F: for<'env> FnOnce(Env<'env>) -> R,
    {
        self.get().with_env(callback)
    }
}

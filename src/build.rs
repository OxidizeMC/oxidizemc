pub struct ModLoaderConfig {
    pub name: &'static str,
    pub lib_name: &'static str,
    pub mod_initializers: &'static [&'static str],
}

pub struct OxidizeMCBuild {
    java_package: Option<&'static str>,
    ml_config: Option<ModLoaderConfig>,
}

impl OxidizeMCBuild {
    pub fn new() -> Self {
        OxidizeMCBuild {
            java_package: None,
            ml_config: None,
        }
    }

    pub fn java_package(mut self, java_package: &'static str) -> Self {
        self.java_package = Some(java_package);
        self
    }

    pub fn mod_loader(mut self, mod_loader_conifg: ModLoaderConfig) -> Self {
        self.ml_config = Some(mod_loader_conifg);
        self
    }

    pub fn finish(mut self) {
        assert!(self.java_package.is_some(), "Configuring the java package name is required");
        println!("cargo::rustc-env=OXIDIZEMC_JAVA_PACKAGE={}", self.java_package.take().unwrap());

        assert!(self.ml_config.is_some(), "Configuring a mod loader is required");
        let ml_config: ModLoaderConfig = self.ml_config.take().unwrap();
        println!("cargo::rustc-env=OXIDIZEMC_MOD_INITIALIZERS={}", ml_config.mod_initializers.join(";"));
    }
}

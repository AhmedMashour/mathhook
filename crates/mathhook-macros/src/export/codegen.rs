use crate::export::{nodejs, python, ExportConfig};
use proc_macro2::TokenStream;
use syn::{ItemFn, Result};

pub fn generate_python_wrapper(func: &ItemFn, config: &ExportConfig) -> Result<TokenStream> {
    python::generate_python_wrapper(func, config)
}

pub fn generate_nodejs_wrapper(func: &ItemFn, config: &ExportConfig) -> Result<TokenStream> {
    nodejs::generate_nodejs_wrapper(func, config)
}

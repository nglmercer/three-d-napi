use napi_derive::napi;

// Module definitions
pub mod context;
pub mod core;
pub mod prelude;

/// A simple greeting function to verify the module is loaded.
#[napi]
pub fn hello_three_d() -> String {
    "three-d N-API bindings loaded successfully!".to_string()
}

use napi_derive::napi;

/// Represents the GPU Context (WebGL/OpenGL).
///
/// In `three-d`, the `Context` struct (often from the `three_d::context` module or exposed via `three_d::Context`)
/// usually requires an actual OpenGL/WebGL implementation (like Glow) to be initialized.
/// Since initializing a GPU context in a headless Node.js environment without a specific
/// windowing system (like Winit or WebGL via JS) is complex and often impossible
/// for a pure Rust library doing direct syscalls, this module uses a mock wrapper.
///
/// This wrapper exposes the N-API structure required for the bindings,
/// while deferring the actual context creation to the host environment
/// (e.g., passing an existing context from Electron/tauri or implementing a headless backend).
#[napi]
pub struct Context {
    /// Internal handle. In a real implementation, this would hold an Arc<three_d::Context>.
    /// For this structural setup, we hold a flag to indicate initialization.
    is_valid: bool,
}

// Context must be Clone to allow holding Arc<Context> in core::Program
impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            is_valid: self.is_valid,
        }
    }
}

#[napi]
impl Context {
    /// Initializes a new GPU Context wrapper.
    /// Note: Actual GPU context creation requires platform-specific setup.
    #[napi(constructor)]
    pub fn new() -> Self {
        // In a full implementation, this would call something like:
        // let inner = three_d::Context::from_gl_context(Arc::new(gl_context)).unwrap();

        Context { is_valid: true }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        "three-d Context (Mocked for N-API bindings)".to_string()
    }
}

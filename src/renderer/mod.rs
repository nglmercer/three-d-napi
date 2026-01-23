use napi_derive::napi;

pub mod control;
pub mod effect;
pub mod geometry;
pub mod light;
pub mod material;
pub mod object;

// Wrapper for Camera
#[napi]
pub struct Camera {
    // impl details
}


#[napi]
impl Camera {
    #[napi(constructor)]
    pub fn new() -> Self {
        Camera {}
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

// Wrapper for Renderer
#[napi]
pub struct Renderer {
    // impl details
}

#[napi]
impl Renderer {
    #[napi(constructor)]
    pub fn new() -> Self {
        Renderer {}
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
pub fn render_with_material() {}

#[napi]
pub fn render_with_effect() {}

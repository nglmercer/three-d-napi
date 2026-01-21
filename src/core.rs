use crate::prelude::{NPoint3, NVector3};
use napi_derive::napi;
use std::sync::Arc;
use three_d::{Camera as ThreeDCamera, Deg, Vector3, Viewport};

/// Represents a 3D Camera.
/// Manages the viewing frustum and perspective/orthographic projection.
#[napi]
pub struct Camera {
    // We hold the camera. Note that three-d Camera does not need the context for its existence,
    // but it usually needs it to update the viewport or calculate matrices.
    inner: ThreeDCamera,
}

#[napi]
impl Camera {
    /// Creates a new perspective camera.
    /// Note: The `aspect` parameter is calculated internally based on the viewport provided.
    #[napi(constructor)]
    pub fn new(
        position: &NPoint3,
        target: &NVector3,
        up: &NVector3,
        fov_degrees: f64,
        near: f64,
        far: f64,
    ) -> Self {
        // Convert N-API types to three-d types
        let position_vec = Vector3::new(position.x as f32, position.y as f32, position.z as f32);
        let target_vec = Vector3::new(target.x as f32, target.y as f32, target.z as f32);
        let up_vec = Vector3::new(up.x as f32, up.y as f32, up.z as f32);

        // Create the inner camera
        // We use a dummy viewport here. In a real application, the viewport is updated
        // every frame or on window resize.
        let viewport = Viewport::new_at_origo(800, 600); // Default size

        let inner = ThreeDCamera::new_perspective(
            viewport,
            // Note: three-d 0.18.2 new_perspective takes (viewport, target, up, position, fov, near, far)
            // This order is deduced from compiler diagnostics.
            target_vec,
            up_vec,
            position_vec,
            Deg(fov_degrees as f32),
            near as f32,
            far as f32,
        );

        Camera { inner }
    }

    /// Returns the camera's position as an array [f64, f64, f64].
    #[napi]
    pub fn get_position(&self) -> Vec<f64> {
        let pos = self.inner.position();
        vec![pos.x as f64, pos.y as f64, pos.z as f64]
    }
}

/// Represents a Shader Program.
/// Placeholder wrapper for the complex program compilation logic in three-d.
#[napi]
pub struct Program {
    _context: Arc<crate::context::Context>,
}

#[napi]
impl Program {
    #[napi(constructor)]
    pub fn new(
        context: &crate::context::Context,
        _vertex_source: String,
        _fragment_source: String,
    ) -> Self {
        // In a real implementation, we would compile shaders here:
        // `ThreeDProgram::from_source(&context.inner, ...).unwrap();`
        // For the structure, we hold the context to prove we are valid.
        Program {
            _context: Arc::new(context.clone()),
        }
    }
}

/// Represents a generic GPU Buffer.
#[napi]
pub struct VertexBuffer {
    pub buffer_type: String, // "Vertex"
    pub name: Option<String>,
    pub vertex_count: u32,
}

#[napi]
impl VertexBuffer {
    #[napi(constructor)]
    pub fn new(name: Option<String>, vertex_count: u32) -> Self {
        VertexBuffer {
            buffer_type: "Vertex".to_string(),
            name,
            vertex_count,
        }
    }
}

/// Represents an Element (Index) Buffer.
#[napi]
pub struct ElementBuffer {
    pub buffer_type: String, // "Element"
    pub name: Option<String>,
    pub index_count: u32,
}

#[napi]
impl ElementBuffer {
    #[napi(constructor)]
    pub fn new(name: Option<String>, index_count: u32) -> Self {
        ElementBuffer {
            buffer_type: "Element".to_string(),
            name,
            index_count,
        }
    }
}

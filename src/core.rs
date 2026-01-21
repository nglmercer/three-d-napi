// Note: core::prelude math types (Point3, Vector3, etc.) can be used internally
// but N-API endpoints use flat coordinates for compatibility
use napi_derive::napi;
use std::sync::Arc;
use three_d::{Camera as ThreeDCamera, Deg, Vector3, Viewport as ThreeDViewport};

// Export buffer module contents
pub mod buffer;

// Render state configuration module
pub mod render_states {
    use napi_derive::napi;

    /// Enable/disable render states for blending, depth, stencil, etc.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct RenderStates {
        pub blend: bool,
        pub blend_src: Option<String>,
        pub blend_dest: Option<String>,
        pub depth_write: bool,
        pub depth_test: bool,
        pub cull_face: Option<String>,
    }

    #[napi]
    impl RenderStates {
        #[napi(constructor)]
        pub fn new(
            blend: Option<bool>,
            blend_src: Option<String>,
            blend_dest: Option<String>,
            depth_write: Option<bool>,
            depth_test: Option<bool>,
            cull_face: Option<String>,
        ) -> Self {
            RenderStates {
                blend: blend.unwrap_or(false),
                blend_src,
                blend_dest,
                depth_write: depth_write.unwrap_or(true),
                depth_test: depth_test.unwrap_or(true),
                cull_face,
            }
        }

        #[napi]
        pub fn default() -> Self {
            RenderStates {
                blend: false,
                blend_src: None,
                blend_dest: None,
                depth_write: true,
                depth_test: true,
                cull_face: None,
            }
        }
    }

    /// Channel write mask for color/depth/stencil buffers.
    #[napi]
    pub enum WriteMask {
        None = 0,
        Color = 1,
        Depth = 2,
        Stencil = 4,
        All = 7,
    }
}

// Reder target module
pub mod render_target {
    use napi_derive::napi;

    /// Buffer clear configuration.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct ClearState {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
        pub alpha: f64,
        pub depth: f64,
        pub stencil: u32,
    }

    #[napi]
    impl ClearState {
        #[napi(constructor)]
        pub fn new(
            red: Option<f64>,
            green: Option<f64>,
            blue: Option<f64>,
            alpha: Option<f64>,
            depth: Option<f64>,
            stencil: Option<u32>,
        ) -> Self {
            ClearState {
                red: red.unwrap_or(0.0),
                green: green.unwrap_or(0.0),
                blue: blue.unwrap_or(0.0),
                alpha: alpha.unwrap_or(1.0),
                depth: depth.unwrap_or(1.0),
                stencil: stencil.unwrap_or(0),
            }
        }
    }

    /// Color render target wrapper.
    #[napi]
    #[derive(Debug)]
    pub struct ColorTarget {
        pub width: u32,
        pub height: u32,
        pub multisample: u32,
    }

    #[napi]
    impl ColorTarget {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, multisample: Option<u32>) -> Self {
            ColorTarget {
                width,
                height,
                multisample: multisample.unwrap_or(0),
            }
        }
    }

    /// Depth render target wrapper.
    #[napi]
    #[derive(Debug)]
    pub struct DepthTarget {
        pub width: u32,
        pub height: u32,
        pub multisample: u32,
    }

    #[napi]
    impl DepthTarget {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, multisample: Option<u32>) -> Self {
            DepthTarget {
                width,
                height,
                multisample: multisample.unwrap_or(0),
            }
        }
    }

    /// Complete render target (frame buffer) wrapper.
    #[napi]
    #[derive(Debug)]
    pub struct RenderTarget {
        pub width: u32,
        pub height: u32,
        pub has_color: bool,
        pub has_depth: bool,
    }

    #[napi]
    impl RenderTarget {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, has_color: bool, has_depth: bool) -> Self {
            RenderTarget {
                width,
                height,
                has_color,
                has_depth,
            }
        }
    }

    /// Multisampled color render target wrapper.
    #[napi]
    #[derive(Debug)]
    pub struct ColorTargetMultisample {
        pub width: u32,
        pub height: u32,
        pub samples: u32,
    }

    #[napi]
    impl ColorTargetMultisample {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, samples: u32) -> Self {
            ColorTargetMultisample {
                width,
                height,
                samples,
            }
        }
    }

    /// Multisampled depth render target wrapper.
    #[napi]
    #[derive(Debug)]
    pub struct DepthTargetMultisample {
        pub width: u32,
        pub height: u32,
        pub samples: u32,
    }

    #[napi]
    impl DepthTargetMultisample {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, samples: u32) -> Self {
            DepthTargetMultisample {
                width,
                height,
                samples,
            }
        }
    }

    /// Multisampled render target.
    #[napi]
    #[derive(Debug)]
    pub struct RenderTargetMultisample {
        pub width: u32,
        pub height: u32,
        pub samples: u32,
    }

    #[napi]
    impl RenderTargetMultisample {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, samples: u32) -> Self {
            RenderTargetMultisample {
                width,
                height,
                samples,
            }
        }
    }
}

/// Represents a 3D Camera.
/// Manages the viewing frustum and perspective/orthographic projection.
/// Uses flat coordinates for N-API compatibility.
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
        position_x: f64,
        position_y: f64,
        position_z: f64,
        target_x: f64,
        target_y: f64,
        target_z: f64,
        up_x: f64,
        up_y: f64,
        up_z: f64,
        fov_degrees: f64,
        near: f64,
        far: f64,
    ) -> Self {
        // Convert flat coordinates to three-d Vector3 types
        let position_vec = Vector3::new(position_x as f32, position_y as f32, position_z as f32);
        let target_vec = Vector3::new(target_x as f32, target_y as f32, target_z as f32);
        let up_vec = Vector3::new(up_x as f32, up_y as f32, up_z as f32);

        // Create the inner camera
        // We use a dummy viewport here. In a real application, the viewport is updated
        // every frame or on window resize.
        let viewport = ThreeDViewport::new_at_origo(800, 600); // Default size

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

/// Represents the Viewport (rendering rectangle area).
/// Defines where rendering occurs on the window/canvas.
#[napi]
#[derive(Clone)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[napi]
impl Viewport {
    /// Creates a new viewport instance.
    #[napi(constructor)]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Viewport {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a viewport at origin (0, 0).
    #[napi]
    pub fn at_origin(width: u32, height: u32) -> Self {
        Viewport {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// Returns aspect ratio (width / height).
    #[napi]
    pub fn aspect_ratio(&self) -> f64 {
        if self.height == 0 {
            1.0
        } else {
            self.width as f64 / self.height as f64
        }
    }

    /// Check if point is within viewport bounds.
    #[napi]
    pub fn contains(&self, px: u32, py: u32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    /// Returns string representation.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "Viewport({},{},{},{})",
            self.x, self.y, self.width, self.height
        )
    }

    /// Converts to internal three-d Viewport.
    pub fn to_internal(&self) -> ThreeDViewport {
        ThreeDViewport::new_at_origo(self.width, self.height)
    }
}

/// Wrapper for Scene Camera - manages camera position and viewport rendering.
/// Note: N-API compatibility requires flat fields instead of nested struct parameters.
#[napi]
pub struct SceneCamera {
    /// Camera position x, y, z
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    /// Camera target/look at x, y, z
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    /// Camera up vector x, y, z
    pub up_x: f64,
    pub up_y: f64,
    pub up_z: f64,
    /// Viewport for camera rendering
    viewport: Viewport,
    /// Field of view in degrees
    #[napi(ts_type = "number")]
    pub fov_degrees: f64,
    /// Near clip plane
    pub near: f64,
    /// Far clip plane
    pub far: f64,
    /// Whether to clear depth buffer before rendering
    pub clear_depth: bool,
    /// Whether to clear color buffer before rendering
    pub clear_color: bool,
    /// Additional clear attributes
    pub clear_buffer: bool,
}

#[napi]
impl SceneCamera {
    /// Creates new SceneCamera with camera parameters (flat coordinates for N-API compatibility).
    #[napi(constructor)]
    pub fn new(
        position_x: f64,
        position_y: f64,
        position_z: f64,
        target_x: f64,
        target_y: f64,
        target_z: f64,
        up_x: f64,
        up_y: f64,
        up_z: f64,
        viewport: &Viewport,
        fov_degrees: Option<f64>,
        near: Option<f64>,
        far: Option<f64>,
        clear_depth: Option<bool>,
        clear_color: Option<bool>,
    ) -> Self {
        SceneCamera {
            position_x,
            position_y,
            position_z,
            target_x,
            target_y,
            target_z,
            up_x,
            up_y,
            up_z,
            viewport: viewport.clone(),
            fov_degrees: fov_degrees.unwrap_or(45.0),
            near: near.unwrap_or(0.1),
            far: far.unwrap_or(1000.0),
            clear_depth: clear_depth.unwrap_or(true),
            clear_color: clear_color.unwrap_or(true),
            clear_buffer: false, // Not implemented
        }
    }

    /// Updates camera projection matrix based on new viewport.
    #[napi]
    pub fn update_viewport(&mut self, viewport: &Viewport) -> () {
        self.viewport = viewport.clone();
    }
    #[napi]
    pub fn get_position(&self) -> Vec<f64> {
        vec![self.position_x, self.position_y, self.position_z]
    }

    /// Returns camera target as [x, y, z].
    #[napi]
    pub fn get_target(&self) -> Vec<f64> {
        vec![self.target_x, self.target_y, self.target_z]
    }

    /// Returns camera up vector as [x, y, z].
    #[napi]
    pub fn get_up(&self) -> Vec<f64> {
        vec![self.up_x, self.up_y, self.up_z]
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "SceneCamera(viewport={}, fov={})",
            self.viewport.get_info(),
            self.fov_degrees
        )
    }
}

// Note: Point2, Point3, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4
// are now defined in prelude.rs with N-API support

/// Represents an Axis-Aligned Bounding Box (AABB).
/// Used for culling and collision detection.
/// Note: N-API compatibility requires flat fields instead of nested structs.
#[napi]
pub struct AxisAlignedBoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

#[napi]
impl AxisAlignedBoundingBox {
    /// Creates a new AABB from min and max coordinates.
    #[napi(constructor)]
    pub fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
        AxisAlignedBoundingBox {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    /// Returns the center point of the AABB as [x, y, z].
    #[napi]
    pub fn center(&self) -> Vec<f64> {
        vec![
            (self.min_x + self.max_x) * 0.5,
            (self.min_y + self.max_y) * 0.5,
            (self.min_z + self.max_z) * 0.5,
        ]
    }

    /// Returns the size (dimensions) of the AABB as [width, height, depth].
    #[napi]
    pub fn size(&self) -> Vec<f64> {
        vec![
            self.max_x - self.min_x,
            self.max_y - self.min_y,
            self.max_z - self.min_z,
        ]
    }

    /// Check if a point is inside the AABB.
    #[napi]
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
            && z >= self.min_z
            && z <= self.max_z
    }

    /// Merges this AABB with another (expands to encompass both).
    #[napi]
    pub fn merge(&self, other: &AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            min_z: self.min_z.min(other.min_z),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
            max_z: self.max_z.max(other.max_z),
        }
    }
}

/// Represents the core viewable 3D mesh, aka a 2D Shape.
/// Render targets for drawing and applying effects.
#[napi]
#[derive(Debug, Clone)]
pub struct ScissorBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[napi]
impl ScissorBox {
    #[napi(constructor)]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        ScissorBox {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, px: u32, py: u32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }
}

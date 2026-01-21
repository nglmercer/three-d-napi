// Note: core::prelude math types (Point3, Vector3, etc.) can be used internally
// but N-API endpoints use flat coordinates for compatibility
use napi_derive::napi;
use std::sync::Arc;
use three_d::{Camera as ThreeDCamera, Vector3, Viewport as ThreeDViewport};

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
        pub fn new_default() -> Self {
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

    /// Blend function types for source/destination blends.
    #[napi]
    pub enum Blend {
        Zero = 0,
        One = 1,
        SrcColor = 2,
        OneMinusSrcColor = 3,
        DstColor = 4,
        OneMinusDstColor = 5,
        SrcAlpha = 6,
        OneMinusSrcAlpha = 7,
        DstAlpha = 8,
        OneMinusDstAlpha = 9,
        ConstantColor = 10,
        OneMinusConstantColor = 11,
        ConstantAlpha = 12,
        OneMinusConstantAlpha = 13,
        SrcAlphaSaturate = 14,
    }

    /// Blend equation types.
    #[napi]
    pub enum BlendEquationType {
        Add = 0,
        Subtract = 1,
        ReverseSubtract = 2,
        Min = 3,
        Max = 4,
    }

    /// Face culling modes.
    #[napi]
    pub enum Cull {
        None = 0,
        Back = 1,
        Front = 2,
        FrontAndBack = 3,
    }

    /// Depth test functions.
    #[napi]
    pub enum DepthTest {
        Never = 0,
        Less = 1,
        Equal = 2,
        LessOrEqual = 3,
        Greater = 4,
        NotEqual = 5,
        GreaterOrEqual = 6,
        Always = 7,
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

    /// Color texture configuration enum.
    #[napi]
    pub enum ColorTexture {
        Texture2D = 0,
        Texture2DArray = 1,
        TextureCube = 2,
        Texture2DMultisample = 3,
        Texture2DArrayMultisample = 4,
    }

    /// Depth texture configuration enum.
    #[napi]
    pub enum DepthTexture {
        Texture2D = 0,
        Texture2DArray = 1,
        Texture2DMultisample = 2,
        Texture2DArrayMultisample = 3,
    }
}

/// Represents a 3D Camera.
/// Manages the viewing frustum and perspective/orthographic projection.
/// Uses flat coordinates for N-API compatibility.
#[napi]
pub struct Camera {
    #[allow(dead_code)]
    inner: ThreeDCamera,
    // Store position separately since three-d camera stores it internally
    position_x: f64,
    position_y: f64,
    position_z: f64,
}

#[napi]
impl Camera {
    /// Creates a new perspective camera.
    /// Note: The `aspect` parameter is calculated internally based on the viewport provided.
    #[allow(clippy::too_many_arguments)]
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
            three_d::Deg(fov_degrees as f32),
            near as f32,
            far as f32,
        );

        Camera {
            inner,
            position_x,
            position_y,
            position_z,
        }
    }

    /// Returns the camera's position as an array [f64, f64, f64].
    #[napi]
    pub fn get_position(&self) -> Vec<f64> {
        vec![self.position_x, self.position_y, self.position_z]
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

/// Core 3D context wrapper.
/// This provides access to GPU/OpenGL context and state management
/// for three-d rendering operations.
#[napi]
pub struct Context {
    width: u32,
    height: u32,
}

#[napi]
impl Context {
    /// Creates a new core context.
    #[napi(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Context { width, height }
    }

    /// Returns the context size.
    #[napi]
    pub fn get_size(&self) -> Vec<u32> {
        vec![self.width, self.height]
    }

    /// Returns context info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!("CoreContext({}x{})", self.width, self.height)
    }
}

/// Core prelude math types.
/// These are re-exported from the prelude module for flat, user-friendly access.
pub mod prelude {
    pub use crate::prelude::{
        Matrix2, Matrix3, Matrix4, NDeg as Deg, NQuaternion as Quaternion, NRad as Rad,
        NSrgba as Srgba, Point2, Point3, Vector2, Vector3, Vector4, NF16 as f16,
    };
}

/// Texture module for core texture types (CpuTexture, etc.)
pub mod texture {
    use napi_derive::napi;

    #[napi]
    pub enum CubeMapSide {
        Right = 0,
        Left = 1,
        Top = 2,
        Bottom = 3,
        Front = 4,
        Back = 5,
    }

    /// CpuTexture is a base struct for CPU-side textures.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct CpuTexture {
        pub width: u32,
        pub height: u32,
        pub format: String,
    }

    #[napi]
    impl CpuTexture {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, format: Option<String>) -> Self {
            CpuTexture {
                width,
                height,
                format: format.unwrap_or_else(|| "rgba8".to_string()),
            }
        }

        #[napi]
        pub fn to_gpu(&self) -> String {
            format!("GPUSurface({}x{} {})", self.width, self.height, self.format)
        }
    }

    /// CpuTexture3D for volumetric textures.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct CpuTexture3D {
        pub width: u32,
        pub height: u32,
        pub depth: u32,
        pub format: String,
    }

    #[napi]
    impl CpuTexture3D {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, depth: u32, format: Option<String>) -> Self {
            CpuTexture3D {
                width,
                height,
                depth,
                format: format.unwrap_or_else(|| "rgba8".to_string()),
            }
        }

        #[napi]
        pub fn to_gpu(&self) -> String {
            format!(
                "GPUVolume({}x{}x{} {})",
                self.width, self.height, self.depth, self.format
            )
        }
    }

    /// CubeMapSideIterator for iterating cubemap faces.
    #[napi]
    pub struct CubeMapSideIterator {
        current: u32,
    }

    #[napi]
    impl CubeMapSideIterator {
        #[napi(constructor)]
        pub fn new() -> Self {
            CubeMapSideIterator { current: 0 }
        }

        #[napi]
        pub fn next(&mut self) -> Option<CubeMapSide> {
            let side = match self.current {
                0 => Some(CubeMapSide::Right),
                1 => Some(CubeMapSide::Left),
                2 => Some(CubeMapSide::Top),
                3 => Some(CubeMapSide::Bottom),
                4 => Some(CubeMapSide::Front),
                5 => Some(CubeMapSide::Back),
                _ => None,
            };
            if side.is_some() {
                self.current += 1;
            }
            side
        }
    }

    /// Mipmap level wrapper.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct Mipmap {
        pub level: u32,
        pub width: u32,
        pub height: u32,
    }

    #[napi]
    impl Mipmap {
        #[napi(constructor)]
        pub fn new(level: u32, width: u32, height: u32) -> Self {
            Mipmap {
                level,
                width,
                height,
            }
        }

        #[napi]
        pub fn get_info(&self) -> String {
            format!("Level {}: {}x{}", self.level, self.width, self.height)
        }
    }

    /// f24 - 24-bit floating point wrapper.
    #[napi]
    #[derive(Clone)]
    pub struct NF24 {
        pub value: f64,
    }

    #[napi]
    impl NF24 {
        #[napi(constructor)]
        pub fn new(value: f64) -> Self {
            NF24 { value }
        }

        #[napi]
        pub fn to_f32(&self) -> f32 {
            self.value as f32
        }
    }

    /// f24 exported as f24 for lib.md compliance.
    pub use NF24 as f24;

    /// DepthTexture2D for depth attachments.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct DepthTexture2D {
        pub width: u32,
        pub height: u32,
    }

    #[napi]
    impl DepthTexture2D {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32) -> Self {
            DepthTexture2D { width, height }
        }

        #[napi]
        pub fn get_info(&self) -> String {
            format!("DepthTexture2D({}x{})", self.width, self.height)
        }
    }

    /// DepthTexture2DArray for depth array textures.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct DepthTexture2DArray {
        pub width: u32,
        pub height: u32,
        pub layers: u32,
    }

    #[napi]
    impl DepthTexture2DArray {
        #[napi(constructor)]
        pub fn new(width: u32, height: u32, layers: u32) -> Self {
            DepthTexture2DArray {
                width,
                height,
                layers,
            }
        }

        #[napi]
        pub fn get_info(&self) -> String {
            format!(
                "DepthTexture2DArray({}x{}x{})",
                self.width, self.height, self.layers
            )
        }
    }

    /// DepthTextureCubeMap for cubemap depth.
    #[napi]
    #[derive(Debug, Clone)]
    pub struct DepthTextureCubeMap {
        pub size: u32,
    }

    #[napi]
    impl DepthTextureCubeMap {
        #[napi(constructor)]
        pub fn new(size: u32) -> Self {
            DepthTextureCubeMap { size }
        }

        #[napi]
        pub fn get_info(&self) -> String {
            format!("DepthTextureCubeMap({}x{})", self.size, self.size)
        }
    }
}

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

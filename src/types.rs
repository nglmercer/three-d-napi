use napi_derive::napi;

// Re-export all enums from the enums module
pub use crate::enums::{
    BackgroundMode, BlendEquation, BlendMultiplier, BufferBindingPoint, BufferStorage, BufferType,
    BufferUsage, ClearFlag, Comparison, CompressionTextureType, CubeMapSide, Cull, CullFace,
    DataType, DepthTest, ExecutionMode, FaceWinding, FenceStatus, FramebufferAttachment,
    GeometryType, GpuQueryType, HardwareAcceleration, HeadlessError, IsophoticModel, LightType,
    MapAccess, MaterialShadingModel, PrimitiveType, QueryResult, RenderStateError, RendererError,
    ShaderType, StableMarkerType, StencilOperation, StereoMode, TessellationMode,
    TextureArrayLayer, TextureFormat, TextureMagFilter, TextureMinFilter, TextureWrap,
    ViewportScaling, WindowError,
};

// Re-export all core types from the core module
pub use crate::core::buffer::{ElementBuffer, InstanceBuffer, UniformBuffer, VertexBuffer};
// Re-export core enums
pub use crate::core::render_states::{Cull as CoreCull, DepthTest as CoreDepthTest};
// Note: Cull and DepthTest in core/ are different from those in enums/

// Re-export all context types
pub use crate::context::{
    ActiveAttribute, ActiveTransformFeedback, ActiveUniform, Context, DebugMessageLogEntry,
    NativeBuffer, NativeFence, NativeFramebuffer, NativeProgram, NativeQuery, NativeRenderbuffer,
    NativeSampler, NativeShader, NativeTexture, NativeTransformFeedback, NativeUniformLocation,
    NativeVertexArray, ProgramBinary, Version,
};

// Re-export all renderer types
pub use crate::renderer::Renderer;

// Re-export all prelude types
pub use crate::prelude::{
    Matrix2, Matrix3, Matrix4, NDeg, NQuaternion, NRad, NSrgba, Point2, Point3, Vector2, Vector3,
    Vector4, NF16,
};

// ============================================================================
// Basic GPU Resource Type Aliases
// ============================================================================

/// Type alias for a GPU buffer handle.
#[napi]
pub type Buffer = NativeBuffer;

/// Type alias for a GPU fence sync object handle.
#[napi]
pub type Fence = NativeFence;

/// Type alias for a GPU framebuffer object handle.
#[napi]
pub type Framebuffer = NativeFramebuffer;

/// Type alias for a GPU program handle.
#[napi]
pub type Program = NativeProgram;

/// Type alias for a GPU query object handle.
#[napi]
pub type Query = NativeQuery;

/// Type alias for a GPU renderbuffer object handle.
#[napi]
pub type Renderbuffer = NativeRenderbuffer;

/// Type alias for a GPU sampler object handle.
#[napi]
pub type Sampler = NativeSampler;

/// Type alias for a GPU shader object handle.
#[napi]
pub type Shader = NativeShader;

/// Type alias for a GPU texture object handle.
#[napi]
pub type Texture = NativeTexture;

/// Type alias for a GPU transform feedback object handle.
#[napi]
pub type TransformFeedback = NativeTransformFeedback;

/// Type alias for a GPU uniform location handle.
#[napi]
pub type UniformLocation = NativeUniformLocation;

/// Type alias for a GPU vertex array object handle.
#[napi]
pub type VertexArray = NativeVertexArray;

// ============================================================================
// Math Type Aliases
// ============================================================================

/// Type alias for a 2D vector.
#[napi]
pub type Vec2 = Vector2;

/// Type alias for a 3D vector.
#[napi]
pub type Vec3 = Vector3;

/// Type alias for a 4D vector.
#[napi]
pub type Vec4 = Vector4;

/// Type alias for a 2x2 matrix.
#[napi]
pub type Mat2 = Matrix2;

/// Type alias for a 3x3 matrix.
#[napi]
pub type Mat3 = Matrix3;

/// Type alias for a 4x4 matrix.
#[napi]
pub type Mat4 = Matrix4;

/// Type alias for a quaternion (rotation).
#[napi]
pub type Quat = NQuaternion;

/// Type alias for degrees rotation.
#[napi]
pub type Degrees = NDeg;

/// Type alias for radians rotation.
#[napi]
pub type Radians = NRad;

/// Type alias for sRGBA color.
#[napi]
pub type Srgba = NSrgba;

/// Type alias for a 2D point (position).
#[napi]
pub type Point2D = Point2;

/// Type alias for a 3D point (position).
#[napi]
pub type Point3D = Point3;

/// Type alias for half-precision float.
#[napi]
pub type Half = NF16;

/// Type alias for geometry bounds
#[napi]
pub type GeometryBounds = [f64; 6];

/// Type alias for frustum culling bounds
#[napi]
pub type FrustumBounds = [f64; 6];

// ============================================================================
// Buffer Type Aliases
// ============================================================================

/// Vertex buffer type.
#[napi]
pub type VertexBufferType = VertexBuffer;

/// Element (index) buffer type.
#[napi]
pub type ElementBufferType = ElementBuffer;

/// Instance buffer type (for instanced rendering).
#[napi]
pub type InstanceBufferType = InstanceBuffer;

/// Uniform buffer type (for uniform blocks).
#[napi]
pub type UniformBufferType = UniformBuffer;

// ============================================================================
// GPU Context and Renderer Type Aliases
// ============================================================================

/// GPU Context type.
#[napi]
pub type GlContext = Context;

/// Renderer type.
#[napi]
pub type GraphicsRenderer = Renderer;

/// Debug message callback function type (JavaScript function).
#[napi]
pub type DebugCallback = String;

// ============================================================================
// Buffer Data Type Aliases
// ============================================================================

/// Geometry buffer type.
#[napi]
pub type GeometryBuffer = Vec<f64>;

/// Index buffer type.
#[napi]
pub type IndexBuffer = Vec<u32>;

/// Attribute buffer type.
#[derive(Debug, Clone)]
pub struct AttributeBuffer(pub Vec<f64>);

/// Uniform buffer data type.
// #[derive(Debug, Clone)]
// pub struct UniformBufferdata(pub Vec<f64>);
#[napi]
pub type UniformBufferdata = Vec<f32>;

// ============================================================================
// Texture Data Type Aliases
// ============================================================================

/// Texture data type.
#[napi]
pub type TextureData = Vec<u8>;

/// Compressed texture data type.
#[napi]
pub type CompressedTextureData = Vec<u8>;

/// Depth texture data type.
#[napi]
pub type DepthTextureData = Vec<u8>;

/// Stencil texture data type.
#[napi]
pub type StencilTextureData = Vec<u8>;

// ============================================================================
// Binary Data Type Aliases
// ============================================================================

/// Binary data type (e.g., program binary).
#[napi]
pub type BinaryData = Vec<u8>;

/// Binary format type.
#[napi]
pub type BinaryFormat = u32;

// ============================================================================
// String Type Aliases
// ============================================================================

/// Type alias for shader source code.
#[napi]
pub type ShaderSource = String;

/// Type alias for shader log.
#[napi]
pub type ShaderLog = String;

/// Type alias for uniform name.
#[napi]
pub type UniformName = String;

/// Type alias for attribute name.
#[napi]
pub type AttributeName = String;

/// Type alias for buffer name.
#[napi]
pub type BufferName = String;

/// Type alias for texture name.
#[napi]
pub type TextureName = String;

/// Type alias for framebuffer name.
#[napi]
pub type FramebufferName = String;

/// Type alias for sampler name.
#[napi]
pub type SamplerName = String;

/// Type alias for program name.
#[napi]
pub type ProgramName = String;

/// Type alias for query name.
#[napi]
pub type QueryName = String;

/// Type alias for transform feedback name.
#[napi]
pub type TransformFeedbackName = String;

/// Type alias for vertex array name.
#[napi]
pub type VertexArrayName = String;

/// Type alias for renderbuffer name.
#[napi]
pub type RenderbufferName = String;

/// Type alias for fence name.
#[napi]
pub type FenceName = String;

/// Type alias for shader name.
#[napi]
pub type ShaderName = String;

/// Type alias for window title.
#[napi]
pub type WindowTitle = String;

/// Type alias for version string.
#[napi]
pub type VersionString = String;

/// Type alias for GLSL version string.
#[napi]
pub type GlslVersionString = String;

/// Type alias for debug message string.
#[napi]
pub type DebugMessage = String;

/// Type alias for info string.
#[napi]
pub type InfoString = String;

// ============================================================================
// Size and Count Type Aliases
// ============================================================================

/// Type alias for buffer size in bytes.
#[napi]
pub type BufferSize = u32;

/// Type alias for buffer offset.
#[napi]
pub type BufferOffset = u32;

/// Type alias for buffer stride.
#[napi]
pub type BufferStride = u32;

/// Type alias for buffer count.
#[napi]
pub type BufferCount = u32;

/// Type alias for vertex count.
#[napi]
pub type VertexCount = u32;

/// Type alias for index count.
#[napi]
pub type IndexCount = u32;

/// Type alias for instance count.
#[napi]
pub type InstanceCount = u32;

/// Type alias for uniform count.
#[napi]
pub type UniformCount = u32;

/// Type alias for attribute count.
#[napi]
pub type AttributeCount = u32;

/// Type alias for texture width.
#[napi]
pub type TextureWidth = u32;

/// Type alias for texture height.
#[napi]
pub type TextureHeight = u32;

/// Type alias for texture depth.
#[napi]
pub type TextureDepth = u32;

// ============================================================================
// ID Type Aliases
// ============================================================================

/// Type alias for buffer ID.
#[napi]
pub type BufferId = u32;

/// Type alias for fence ID.
#[napi]
pub type FenceId = i64;

/// Type alias for framebuffer ID.
#[napi]
pub type FboId = u32;

/// Type alias for program ID.
#[napi]
pub type ProgramId = u32;

/// Type alias for query ID.
#[napi]
pub type QueryId = u32;

/// Type alias for renderbuffer ID.
#[napi]
pub type RenderbufferId = u32;

/// Type alias for sampler ID.
#[napi]
pub type SamplerId = u32;

/// Type alias for shader ID.
#[napi]
pub type ShaderId = u32;

/// Type alias for texture ID.
#[napi]
pub type TextureId = u32;

/// Type alias for transform feedback ID.
#[napi]
pub type TransformFeedbackId = u32;

/// Type alias for uniform location ID.
#[napi]
pub type UniformLocationId = i32;

/// Type alias for vertex array ID.
#[napi]
pub type VertexArrayId = u32;

/// Type alias for query result value.
#[napi]
pub type QueryResultValue = i64;

// ============================================================================
// GL Constant Type Aliases
// ============================================================================

/// Type alias for GL constant.
#[napi]
pub type GlConstant = u32;

/// Type alias for GL hint.
#[napi]
pub type GlHint = u32;

/// Type alias for GL error code.
#[napi]
pub type GlErrorCode = u32;

// ============================================================================
// Viewport and Scissor Type Aliases
// ============================================================================

/// Type alias for viewport X coordinate.
#[napi]
pub type ViewportX = i32;

/// Type alias for viewport Y coordinate.
#[napi]
pub type ViewportY = i32;

/// Type alias for viewport width.
#[napi]
pub type ViewportWidth = u32;

/// Type alias for viewport height.
#[napi]
pub type ViewportHeight = u32;

/// Type alias for scissor X coordinate.
#[napi]
pub type ScissorX = i32;

/// Type alias for scissor Y coordinate.
#[napi]
pub type ScissorY = i32;

/// Type alias for scissor width.
#[napi]
pub type ScissorWidth = u32;

/// Type alias for scissor height.
#[napi]
pub type ScissorHeight = u32;

// ============================================================================
// Clear and Stencil Type Aliases
// ============================================================================

/// Type alias for clear stencil.
#[napi]
pub type ClearStencil = i32;

// ============================================================================
// Sync and Timing Type Aliases
// ============================================================================

/// Type alias for timestamp.
#[napi]
pub type Timestamp = u64;

/// Type alias for timeout.
#[napi]
pub type Timeout = u64;

/// Type alias for sync object.
#[napi]
pub type SyncObject = i64;

// ============================================================================
// Mask and Capability Type Aliases
// ============================================================================

/// Type alias for mask.
#[napi]
pub type Mask = u32;

/// Type alias for capability.
#[napi]
pub type Capability = u32;

// ============================================================================
// Hint Type Aliases
// ============================================================================

/// Type alias for draw mode hint (matches enum definition).
#[napi]
pub type DrawModeHint = crate::enums::DrawModeHint;

/// Type alias for hint target.
#[napi]
pub type HintTarget = u32;

/// Type alias for hint mode.
#[napi]
pub type HintMode = u32;

// ============================================================================
// Pixel Type Aliases
// ============================================================================

/// Type alias for pixel format.
#[napi]
pub type PixelFormat = u32;

/// Type alias for pixel type.
#[napi]
pub type PixelType = u32;

// ============================================================================
// Target Type Aliases (Framebuffers, Textures, Renderbuffers, Samplers, Buffers)
// ============================================================================

/// Type alias for framebuffer target.
#[napi]
pub type FramebufferTarget = u32;

/// Type alias for renderbuffer target.
#[napi]
pub type RenderbufferTarget = u32;

/// Type alias for texture target.
#[napi]
pub type TextureTarget = u32;

/// Type alias for sampler target.
#[napi]
pub type SamplerTarget = u32;

/// Type alias for buffer target.
#[napi]
pub type BufferTarget = u32;

// ============================================================================
// Access and Parameter Type Aliases
// ============================================================================

/// Type alias for buffer access.
#[napi]
pub type BufferAccess = u32;

/// Type alias for buffer map access.
#[napi]
pub type BufferMapAccess = u32;

/// Type alias for buffer parameter.
#[napi]
pub type BufferParameter = u32;

/// Type alias for texture parameter.
#[napi]
pub type TextureParameter = u32;

/// Type alias for sampler parameter.
#[napi]
pub type SamplerParameter = u32;

/// Type alias for framebuffer parameter.
#[napi]
pub type FramebufferParameter = u32;

/// Type alias for renderbuffer parameter.
#[napi]
pub type RenderbufferParameter = u32;

/// Type alias for program parameter.
#[napi]
pub type ProgramParameter = u32;

/// Type alias for shader parameter.
#[napi]
pub type ShaderParameter = u32;

/// Type alias for vertex array parameter.
#[napi]
pub type VertexArrayParameter = u32;

/// Type alias for transform feedback parameter.
#[napi]
pub type TransformFeedbackParameter = u32;

/// Type alias for query parameter.
#[napi]
pub type QueryParameter = u32;

/// Type alias for fence parameter.
#[napi]
pub type FenceParameter = u32;

// ============================================================================
// Debug Type Aliases
// ============================================================================

/// Type alias for debug message parameter.
#[napi]
pub type DebugMessageParameter = u32;

/// Type alias for debug source.
#[napi]
pub type DebugSource = u32;

/// Type alias for debug type.
#[napi]
pub type DebugType = u32;

/// Type alias for debug severity.
#[napi]
pub type DebugSeverity = u32;

/// Type alias for debug message ID.
#[napi]
pub type DebugMessageId = u32;

/// Type alias for program binary format.
#[napi]
pub type ProgramBinaryFormat = u32;

// ============================================================================
// Debug Message (Extended)
// ============================================================================

/// Debugger message type.
#[napi]
pub type DebuggerMessage = String;

// ============================================================================
// Render State Descriptor
// ============================================================================

/// Render state descriptor structure - contains all rendering state configuration.
#[napi]
pub struct RenderStateDescriptor {
    /// Whether blending is enabled.
    pub blending_enabled: bool,
    /// Blending equation.
    pub blend_equation: BlendEquation,
    /// Source blend factor.
    pub blend_src: BlendMultiplier,
    /// Destination blend factor.
    pub blend_dst: BlendMultiplier,
    /// Whether depth test is enabled.
    pub depth_test_enabled: bool,
    /// Depth comparison function.
    pub depth_func: Comparison,
    /// Whether depth buffer writing is enabled.
    pub depth_write_mask: bool,
    /// Whether stencil test is enabled.
    pub stencil_test_enabled: bool,
    /// Stencil comparison function.
    pub stencil_func: Comparison,
    /// Stencil reference value.
    pub stencil_ref: i32,
    /// Stencil mask value.
    pub stencil_mask: u32,
    /// Stencil operation on stencil fail.
    pub stencil_fail: StencilOperation,
    /// Stencil operation on depth fail.
    pub stencil_z_fail: StencilOperation,
    /// Stencil operation on depth pass.
    pub stencil_z_pass: StencilOperation,
    /// Face to cull.
    pub cull_face: CullFace,
    /// Front face winding order.
    pub front_face: FaceWinding,
    /// Polygon drawing mode.
    pub polygon_mode: crate::enums::PolygonMode,
    /// Whether alpha to coverage is enabled.
    pub alpha_to_coverage: bool,
    /// Whether dithering is enabled.
    pub dither: bool,
    /// Whether scissor test is enabled.
    pub scissor_test: bool,
    /// Scissor rectangle coordinates.
    pub scissor_x: i32,
    pub scissor_y: i32,
    pub scissor_width: u32,
    pub scissor_height: u32,
    /// Viewport coordinates.
    pub viewport_x: i32,
    pub viewport_y: i32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    /// Clear color components.
    pub clear_color_r: f64,
    pub clear_color_g: f64,
    pub clear_color_b: f64,
    pub clear_color_a: f64,
    /// Clear depth value.
    pub clear_depth: f64,
    /// Clear stencil value.
    pub clear_stencil: i32,
    /// Clear mask.
    pub clear_mask: u32,
}

// ============================================================================
// Clear Mask Enum
// ============================================================================

/// Framebuffer clear mask.
#[napi]
#[derive(Debug, Clone)]
pub enum ClearMask {
    /// Clear color buffer bit.
    Color = 0x00004000, // GL_COLOR_BUFFER_BIT
    /// Clear depth buffer bit.
    Depth = 0x00000100, // GL_DEPTH_BUFFER_BIT
    /// Clear stencil buffer bit.
    Stencil = 0x00000400, // GL_STENCIL_BUFFER_BIT
}

// ============================================================================
// Buffer Usage Alias
// ============================================================================

/// Buffer usage hint type (matches enum definition).
#[napi]
pub type BufferUsageHint = crate::enums::BufferUsage;

// ============================================================================
// Core Error Type Alias
// ============================================================================

/// Core error type (for compatibility).
#[napi]
pub type CoreError = u32;

// ============================================================================
// Implementation for RenderStateDescriptor
// ============================================================================

#[napi]
impl Default for RenderStateDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl RenderStateDescriptor {
    /// Creates a new render state descriptor with default values.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            blending_enabled: false,
            blend_equation: BlendEquation::Add,
            blend_src: BlendMultiplier::One,
            blend_dst: BlendMultiplier::Zero,
            depth_test_enabled: false,
            depth_func: Comparison::Less,
            depth_write_mask: true,
            stencil_test_enabled: false,
            stencil_func: Comparison::Always,
            stencil_ref: 0,
            stencil_mask: 0xFF,
            stencil_fail: StencilOperation::Keep,
            stencil_z_fail: StencilOperation::Keep,
            stencil_z_pass: StencilOperation::Keep,
            cull_face: CullFace::None,
            front_face: FaceWinding::CounterClockwise,
            polygon_mode: crate::enums::PolygonMode::Fill,
            alpha_to_coverage: false,
            dither: false,
            scissor_test: false,
            scissor_x: 0,
            scissor_y: 0,
            scissor_width: 0,
            scissor_height: 0,
            viewport_x: 0,
            viewport_y: 0,
            viewport_width: 0,
            viewport_height: 0,
            clear_color_r: 0.0,
            clear_color_g: 0.0,
            clear_color_b: 0.0,
            clear_color_a: 1.0,
            clear_depth: 1.0,
            clear_stencil: 0,
            clear_mask: 0,
        }
    }
}

use napi_derive::napi;

/// Core GPU context and state management.
/// These provide access to GPU/OpenGL contexts, state queries, and native resource handles.
///
/// # Example
/// ```javascript
/// const { Context } = require('three-d-napi');
/// const ctx = new Context();
/// console.log(ctx.get_info());
/// ```
pub mod context;

/// Core 3D rendering types.
/// Contains camera, program, viewport, and rendering utilities.
///
/// # Modules
/// * `buffer` - GPU buffer management (VertexBuffer, ElementBuffer, InstanceBuffer, UniformBuffer)
/// * `render_states` - Render state configuration (blend, depth, stencil, cull)
/// * `render_target` - Frame buffer objects and rendering targets
/// * `texture` - Advanced texture types (3D, cubemap, depth textures)
pub mod core;

/// Mathematical prelude types.
/// Provides common 3D math types: points, vectors, matrices, quaternions, angles.
///
/// # Types
/// * `Point2`, `Point3` - 2D and 3D points
/// * `Vector2`, `Vector3`, `Vector4` - Vectors for direction/normal/color
/// * `Matrix2`, `Matrix3`, `Matrix4` - 2x2, 3x3, 4x4 matrices
/// * `Deg`, `Rad` - Angle types (degrees and radians)
/// * `Quaternion` - Quaternion for rotations
pub mod prelude;

/// Simple greeting function to verify the module is loaded.
#[napi]
pub fn hello_three_d() -> String {
    "three-d N-API bindings loaded successfully!".to_string()
}

/// Returns version information for the three-d crate and N-API bindings.
#[napi]
pub fn get_version() -> String {
    "three-d: 0.18.2, N-API Bindings: 0.1.0".to_string()
}

/// Re-exports for backward compatibility and to match lib.md documentation structure.
/// These export types from submodule paths to match the documented API structure.
// Context module exports
pub use context::{
    ActiveAttribute, ActiveTransformFeedback, ActiveUniform, Context, DebugMessageLogEntry,
    NativeBuffer, NativeFence, NativeFramebuffer, NativeProgram, NativeQuery, NativeRenderbuffer,
    NativeSampler, NativeShader, NativeTexture, NativeTransformFeedback, NativeUniformLocation,
    NativeVertexArray, ProgramBinary, Version,
};

// Core module exports (including core::Context, AxisAlignedBoundingBox)
pub use core::{
    AxisAlignedBoundingBox, Camera, Context as CoreContext, Program, ScissorBox, Viewport,
};

// Core::prelude math types (to match lib.md documentation)
pub use core::prelude::{
    f16, Deg, Matrix2, Matrix3, Matrix4, Point2, Point3, Quaternion, Rad, Srgba, Vector2, Vector3,
    Vector4,
};

// Core::buffer exports
pub use core::buffer::{ElementBuffer, InstanceBuffer, UniformBuffer, VertexBuffer};

// Core::render_states exports
pub use core::render_states::{Blend, BlendEquationType, Cull, DepthTest, RenderStates, WriteMask};

// Core::render_target exports
pub use core::render_target::{
    ClearState, ColorTarget, ColorTargetMultisample, ColorTexture, DepthTarget,
    DepthTargetMultisample, DepthTexture, RenderTarget, RenderTargetMultisample,
};

// Core::texture exports (low‑level texture utilities)
pub use core::texture::{
    CpuTexture, CpuTexture3D, CubeMapSide, CubeMapSideIterator, DepthTexture2D,
    DepthTexture2DArray, DepthTextureCubeMap, Mipmap, NF24 as f24,
};

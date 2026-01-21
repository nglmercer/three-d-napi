use napi_derive::napi;

/// Represents the GPU Context (WebGL/OpenGL).
///
/// In three-d, the Context struct requires an actual OpenGL/WebGL implementation
/// (like Glow) to be initialized. Since initializing a GPU context in a headless
/// Node.js environment without a specific windowing system (like Winit or WebGL via JS)
/// is complex, this module uses a mock wrapper.
///
/// This wrapper exposes the N-API structure required for the bindings,
/// while deferring the actual context creation to the host environment
/// (e.g., passing an existing context from Electron/tauri or implementing a headless backend).
#[napi]
pub struct Context {
    /// Internal handle. In a real implementation, this would hold an Arc<three_d::Context>.
    /// For this structural setup, we hold a flag to indicate initialization.
    is_valid: bool,
    /// Cached version info
    version: Option<Version>,
}

// Context must be Clone to allow holding Arc<Context> in core::Program
impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            is_valid: self.is_valid,
            version: self.version.clone(),
        }
    }
}

#[napi]
impl Context {
    /// Initializes a new GPU Context wrapper.
    /// Note: Actual GPU context creation requires platform-specific setup.
    #[napi(constructor)]
    pub fn new() -> Self {
        Context {
            is_valid: true,
            version: Some(Version::new(3, 2, None, None)),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        if self.is_valid {
            "three-d Context (Mocked for N-API bindings)".to_string()
        } else {
            "three-d Context (Invalid)".to_string()
        }
    }

    #[napi]
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    #[napi]
    pub fn get_version(&self) -> Option<Version> {
        self.version.clone()
    }

    /// Creates a version object from major/minor/patch numbers
    #[napi]
    pub fn create_version(major: u8, minor: u8, patch: u8, is_webgl: bool) -> Version {
        Version::new(major, minor, Some(patch), Some(is_webgl))
    }

    /// Returns the current GLSL version supported
    #[napi]
    pub fn get_glsl_version(&self) -> String {
        // Mock GLSL version based on context version
        match &self.version {
            Some(v) if v.major >= 3 => format!("GLSL {}.{}0", v.major, v.minor),
            Some(v) => format!("GLSL {}.{}0", v.major, v.minor),
            None => "GLSL 1.20".to_string(),
        }
    }
}

/// Native GL/Low-level Buffer handle wrapper for GPU operations.
#[napi]
pub struct NativeBuffer {
    /// Native GPU buffer ID
    pub buffer_id: u32,
    /// Type of buffer (e.g., GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER)
    #[napi(ts_type = "number")]
    pub buffer_type: u32,
    /// Size in bytes
    pub size_bytes: u32,
    /// Whether the buffer is currently bound
    pub is_bound: bool,
}

#[napi]
impl NativeBuffer {
    #[napi(constructor)]
    pub fn new(buffer_id: u32, buffer_type: u32, size_bytes: u32, is_bound: Option<bool>) -> Self {
        NativeBuffer {
            buffer_id,
            buffer_type,
            size_bytes,
            is_bound: is_bound.unwrap_or(false),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeBuffer(id={}, type={}, size={} bytes)",
            self.buffer_id, self.buffer_type, self.size_bytes
        )
    }
}

/// Native GPU Fence sync object wrapper.
#[napi]
pub struct NativeFence {
    /// Native fence ID (64-bit for OpenGL sync objects)
    /// Note: u64 is not supported in napi, using i64 instead
    pub fence_id: i64,
    /// Whether the fence has been signaled
    #[napi(ts_type = "boolean")]
    pub signaled: bool,
}

#[napi]
impl NativeFence {
    #[napi(constructor)]
    pub fn new(fence_id: i64, signaled: Option<bool>) -> Self {
        NativeFence {
            fence_id,
            signaled: signaled.unwrap_or(false),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeFence(id={}, signaled={})",
            self.fence_id, self.signaled
        )
    }
}

/// Native Framebuffer Object (FBO) wrapper.
#[napi]
pub struct NativeFramebuffer {
    /// Native FBO ID
    pub fbo_id: u32,
    /// Whether the framebuffer is currently bound to target
    #[napi(ts_type = "boolean")]
    pub is_bound: bool,
    /// Width of the framebuffer attachment
    pub width: u32,
    /// Height of the framebuffer attachment
    pub height: u32,
}

#[napi]
impl NativeFramebuffer {
    #[napi(constructor)]
    pub fn new(
        fbo_id: u32,
        is_bound: Option<bool>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Self {
        NativeFramebuffer {
            fbo_id,
            is_bound: is_bound.unwrap_or(false),
            width: width.unwrap_or(0),
            height: height.unwrap_or(0),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeFramebuffer(id={}, bound={}, {}x{})",
            self.fbo_id, self.is_bound, self.width, self.height
        )
    }
}

/// Native OpenGL Program handle wrapper.
#[napi]
pub struct NativeProgram {
    /// Native program ID
    pub program_id: u32,
    /// Whether the program is valid and linked
    #[napi(ts_type = "boolean")]
    pub is_valid: bool,
    /// Number of attributes in the program
    pub attribute_count: u32,
}

#[napi]
impl NativeProgram {
    #[napi(constructor)]
    pub fn new(program_id: u32, is_valid: Option<bool>, attribute_count: Option<u32>) -> Self {
        NativeProgram {
            program_id,
            is_valid: is_valid.unwrap_or(false),
            attribute_count: attribute_count.unwrap_or(0),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeProgram(id={}, valid={}, attrs={})",
            self.program_id, self.is_valid, self.attribute_count
        )
    }
}

/// Native GPU Query (Occlusion/Timestamp) wrapper.
#[napi]
pub struct NativeQuery {
    /// Native query ID
    pub query_id: u32,
    /// Type of query (GL_ANY_SAMPLES_PASSED, GL_TIME_ELAPSED, etc.)
    #[napi(ts_type = "number")]
    pub query_type: u32,
    /// Result value (set after query is completed)
    /// Note: u64 is not supported in napi, using i64 instead
    pub result: Option<i64>,
}

#[napi]
impl NativeQuery {
    #[napi(constructor)]
    pub fn new(query_id: u32, query_type: u32, result: Option<i64>) -> Self {
        NativeQuery {
            query_id,
            query_type,
            result,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeQuery(id={}, type={}, result={:?})",
            self.query_id, self.query_type, self.result
        )
    }
}

/// Native Renderbuffer Object (RBO) wrapper.
#[napi]
pub struct NativeRenderbuffer {
    /// Native RBO ID
    pub rbo_id: u32,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Internal format (e.g., GL_DEPTH_COMPONENT24)
    #[napi(ts_type = "number")]
    pub internal_format: u32,
}

#[napi]
impl NativeRenderbuffer {
    #[napi(constructor)]
    pub fn new(rbo_id: u32, width: u32, height: u32, internal_format: u32) -> Self {
        NativeRenderbuffer {
            rbo_id,
            width,
            height,
            internal_format,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeRenderbuffer(id={}, {}x{}, format={})",
            self.rbo_id, self.width, self.height, self.internal_format
        )
    }
}

/// Native OpenGL Sampler wrapper for managing texture filtering/sampling.
#[napi]
pub struct NativeSampler {
    /// Native sampler ID
    pub sampler_id: u32,
    /// Wrapping mode for S coordinate (GL_REPEAT, GL_CLAMP_TO_EDGE)
    #[napi(ts_type = "number")]
    pub wrapping_s: u32,
    /// Wrapping mode for T coordinate
    #[napi(ts_type = "number")]
    pub wrapping_t: u32,
    /// Minification filter (GL_LINEAR, GL_NEAREST, etc.)
    #[napi(ts_type = "number")]
    pub min_filter: u32,
    /// Magnification filter
    #[napi(ts_type = "number")]
    pub mag_filter: u32,
}

#[napi]
impl NativeSampler {
    #[napi(constructor)]
    pub fn new(
        sampler_id: u32,
        wrapping_s: u32,
        wrapping_t: u32,
        min_filter: u32,
        mag_filter: u32,
    ) -> Self {
        NativeSampler {
            sampler_id,
            wrapping_s,
            wrapping_t,
            min_filter,
            mag_filter,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeSampler(id={}, wrap(S,T)=({},{}, filt=({},{})",
            self.sampler_id, self.wrapping_s, self.wrapping_t, self.min_filter, self.mag_filter
        )
    }
}

/// Native Shader Object wrapper.
#[napi]
pub struct NativeShader {
    /// Native shader ID
    pub shader_id: u32,
    /// Type of shader (GL_VERTEX_SHADER, GL_FRAGMENT_SHADER, etc.)
    #[napi(ts_type = "number")]
    pub shader_type: u32,
    /// Source code length in bytes
    pub source_len: u32,
}

#[napi]
impl NativeShader {
    #[napi(constructor)]
    pub fn new(shader_id: u32, shader_type: u32, source_len: Option<u32>) -> Self {
        NativeShader {
            shader_id,
            shader_type,
            source_len: source_len.unwrap_or(0),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeShader(id={}, type={}, src_len={})",
            self.shader_id, self.shader_type, self.source_len
        )
    }
}

/// Native OpenGL Texture handle wrapper.
#[napi]
pub struct NativeTexture {
    /// Native texture ID
    pub texture_id: u32,
    /// Type of texture (GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, etc.)
    #[napi(ts_type = "number")]
    pub texture_type: u32,
    /// Width in texels
    pub width: u32,
    /// Height in texels
    pub height: u32,
    /// Internal format (e.g., GL_RGBA8, GL_DEPTH_COMPONENT32F)
    #[napi(ts_type = "number")]
    pub internal_format: u32,
}

#[napi]
impl NativeTexture {
    #[napi(constructor)]
    pub fn new(
        texture_id: u32,
        texture_type: u32,
        width: u32,
        height: u32,
        internal_format: u32,
    ) -> Self {
        NativeTexture {
            texture_id,
            texture_type,
            width,
            height,
            internal_format,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeTexture(id={}, type={}, {}x{}, format={})",
            self.texture_id, self.texture_type, self.width, self.height, self.internal_format
        )
    }
}

/// Native Transform Feedback Object wrapper.
#[napi]
pub struct NativeTransformFeedback {
    /// Native TF ID
    pub tf_id: u32,
    /// Whether currently bound
    #[napi(ts_type = "boolean")]
    pub is_bound: bool,
}

#[napi]
impl NativeTransformFeedback {
    #[napi(constructor)]
    pub fn new(tf_id: u32, is_bound: Option<bool>) -> Self {
        NativeTransformFeedback {
            tf_id,
            is_bound: is_bound.unwrap_or(false),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeTransformFeedback(id={}, bound={})",
            self.tf_id, self.is_bound
        )
    }
}

/// Native Uniform Location wrapper (used for uniform setting operations).
#[napi]
pub struct NativeUniformLocation {
    /// Location ID within the program
    pub location: i32,
    /// Associated program ID for verification
    pub program_id: u32,
    /// Name of the uniform (for debugging)
    pub name: Option<String>,
}

#[napi]
impl NativeUniformLocation {
    #[napi(constructor)]
    pub fn new(location: i32, program_id: u32, name: Option<String>) -> Self {
        NativeUniformLocation {
            location,
            program_id,
            name,
        }
    }

    #[napi]
    pub fn is_valid(&self) -> bool {
        self.location != -1
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeUniformLocation(prog={}, loc={}, name={:?})",
            self.program_id, self.location, self.name
        )
    }
}

/// Native OpenGL Vertex Array Object (VAO) wrapper.
#[napi]
pub struct NativeVertexArray {
    /// Native VAO ID
    pub vao_id: u32,
    /// Whether currently bound to GL target
    #[napi(ts_type = "boolean")]
    pub is_bound: bool,
    /// Number of vertex attributes
    pub attribute_count: u32,
}

#[napi]
impl NativeVertexArray {
    #[napi(constructor)]
    pub fn new(vao_id: u32, is_bound: Option<bool>, attribute_count: Option<u32>) -> Self {
        NativeVertexArray {
            vao_id,
            is_bound: is_bound.unwrap_or(false),
            attribute_count: attribute_count.unwrap_or(0),
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "NativeVertexArray(id={}, bound={}, attrs={})",
            self.vao_id, self.is_bound, self.attribute_count
        )
    }
}

/// Represent active attribute information from a shader program.
#[napi]
pub struct ActiveAttribute {
    /// Name of the attribute
    pub name: String,
    /// Type of the attribute (e.g., GL_FLOAT, GL_FLOAT_VEC3)
    #[napi(ts_type = "number")]
    pub attribute_type: u32,
    /// Size of the attribute (e.g., 1 for float, 3 for vec3)
    pub size: i32,
}

#[napi]
impl ActiveAttribute {
    #[napi(constructor)]
    pub fn new(name: String, attribute_type: u32, size: i32) -> Self {
        ActiveAttribute {
            name,
            attribute_type,
            size,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "ActiveAttribute(name={}, type={}, size={})",
            self.name, self.attribute_type, self.size
        )
    }
}

/// Represent active uniform information from a shader program.
#[napi]
pub struct ActiveUniform {
    /// Name of the uniform
    pub name: String,
    /// Type of the uniform (e.g., GL_FLOAT, GL_FLOAT_VEC3, GL_FLOAT_MAT4)
    #[napi(ts_type = "number")]
    pub uniform_type: u32,
    /// Size of the uniform (array size, 1 for scalars)
    pub size: i32,
}

#[napi]
impl ActiveUniform {
    #[napi(constructor)]
    pub fn new(name: String, uniform_type: u32, size: i32) -> Self {
        ActiveUniform {
            name,
            uniform_type,
            size,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "ActiveUniform(name={}, type={}, size={})",
            self.name, self.uniform_type, self.size
        )
    }
}

/// Represent active transform feedback varying information.
#[napi]
pub struct ActiveTransformFeedback {
    /// Name of the transform feedback varying
    pub name: String,
    /// Buffer binding point index
    pub buffer_binding: u32,
    /// Size (number of components)
    pub size: i32,
}

#[napi]
impl ActiveTransformFeedback {
    #[napi(constructor)]
    pub fn new(name: String, buffer_binding: u32, size: i32) -> Self {
        ActiveTransformFeedback {
            name,
            buffer_binding,
            size,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "ActiveTransformFeedback(name={}, binding={}, size={})",
            self.name, self.buffer_binding, self.size
        )
    }
}

/// Debug message log entry from OpenGL debug output.
#[napi]
pub struct DebugMessageLogEntry {
    /// Source of the message (e.g., API, ShaderCompiler, ThirdParty)
    #[napi(ts_type = "number")]
    pub source: u32,
    /// Type of message (Error, Deprecated, Undefined, Performance, Other)
    #[napi(ts_type = "number")]
    pub message_type: u32,
    /// Identifier of the message
    pub message_id: u32,
    /// Severity of the message (High, Medium, Low, Notification)
    #[napi(ts_type = "number")]
    pub severity: u32,
    /// The actual message text
    pub message: String,
}

#[napi]
impl DebugMessageLogEntry {
    #[napi(constructor)]
    pub fn new(
        source: u32,
        message_type: u32,
        message_id: u32,
        severity: u32,
        message: String,
    ) -> Self {
        DebugMessageLogEntry {
            source,
            message_type,
            message_id,
            severity,
            message,
        }
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "DebugMsg(src={}, type={}, id={}, sev={}: {})",
            self.source, self.message_type, self.message_id, self.severity, self.message
        )
    }
}

/// Serialized whatwg link of a GPU Program (binary format).
#[napi]
pub struct ProgramBinary {
    /// Binary data of the program
    pub binary: Vec<u8>,
    /// Format of the binary (e.g., GL_NVIDIA_BINARY_FORMAT, GL_SPV_SPIR_V)
    #[napi(ts_type = "number")]
    pub format: u32,
}

#[napi]
impl ProgramBinary {
    #[napi(constructor)]
    pub fn new(binary: Vec<u8>, format: u32) -> Self {
        ProgramBinary { binary, format }
    }

    #[napi]
    pub fn length(&self) -> usize {
        self.binary.len()
    }

    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "ProgramBinary(format={}, bytes={})",
            self.format,
            self.binary.len()
        )
    }
}

/// OpenGL/GPU Context Version information.
#[napi]
#[derive(Clone, Debug)]
pub struct Version {
    /// Major version (e.g., 3 for OpenGL 3.2)
    pub major: u8,
    /// Minor version (e.g., 2 for OpenGL 3.2)
    pub minor: u8,
    /// Patch version (rarely used, default 0)
    pub patch: u8,
    /// Whether this is a WebGL context
    pub is_webgl: bool,
}

#[napi]
impl Version {
    #[napi(constructor)]
    pub fn new(major: u8, minor: u8, patch: Option<u8>, is_webgl: Option<bool>) -> Self {
        Version {
            major,
            minor,
            patch: patch.unwrap_or(0),
            is_webgl: is_webgl.unwrap_or(false),
        }
    }

    #[napi]
    pub fn to_string(&self) -> String {
        if self.is_webgl {
            format!("WebGL {}.{}", self.major, self.minor)
        } else {
            let patch_str = if self.patch > 0 {
                format!(".{}", self.patch)
            } else {
                String::new()
            };
            format!("OpenGL {}.{}{}", self.major, self.minor, patch_str)
        }
    }

    /// Returns a valid GLSL version string for this context version
    #[napi]
    pub fn glsl_target(&self) -> String {
        match (self.major, self.minor) {
            (3, 0) => "GLSL 1.30".to_string(),
            (3, 1) => "GLSL 1.40".to_string(),
            (3, 2) => "GLSL 1.50".to_string(),
            (3, 3) => "GLSL 3.30".to_string(),
            (4, 0) => "GLSL 4.00".to_string(),
            (4, 1) => "GLSL 4.10".to_string(),
            (4, 2) => "GLSL 4.20".to_string(),
            (4, 3) => "GLSL 4.30".to_string(),
            (4, 4) => "GLSL 4.40".to_string(),
            (4, 5) => "GLSL 4.50".to_string(),
            (4, 6) => "GLSL 4.60".to_string(),
            _ => {
                if self.major >= 4 {
                    format!("GLSL {}.{}0", self.major, self.minor)
                } else {
                    format!("GLSL {}.{}0", self.major, self.minor)
                }
            }
        }
    }

    /// Check if version supports a specific feature
    #[napi]
    pub fn supports(&self, major: u8, minor: u8) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

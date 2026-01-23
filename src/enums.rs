use napi_derive::napi;

/// Buffer usage hint for GPU buffers.
/// Determines how the buffer data will be used.
#[napi]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    StaticDraw = 0x88E4,  // GL_STATIC_DRAW
    DynamicDraw = 0x88E8, // GL_DYNAMIC_DRAW
    StreamDraw = 0x88E0,  // GL_STREAM_DRAW
    StaticRead = 0x88E5,  // GL_STATIC_READ
    DynamicRead = 0x88E9, // GL_DYNAMIC_READ
    StreamRead = 0x88E1,  // GL_STREAM_READ
    StaticCopy = 0x88E6,  // GL_STATIC_COPY
    DynamicCopy = 0x88EA, // GL_DYNAMIC_COPY
    StreamCopy = 0x88E2,  // GL_STREAM_COPY
}

/// Generic buffer type classification.
#[napi]
#[derive(Debug, Clone)]
pub enum BufferType {
    Vertex,
    Element,
    Instance,
    Uniform,
}

/// Stable marker types used in chunk sets.
#[napi]
#[derive(Debug, Clone)]
pub enum StableMarkerType {
    None,
    Unit,
    Unit2,
    Unit3,
    Unit4,
    Unit2x2,
    Unit3x3,
    Unit4x4,
    Unit2x3,
    Unit2x4,
    Unit3x2,
    Unit3x4,
    Unit4x2,
    Unit4x3,
}

/// Shading model types for materials.
#[napi]
#[derive(Debug, Clone)]
pub enum IsophoticModel {
    CookTorrance,
    OrenNayar,
    Beckmann,
    BlinnPhong,
    Phong,
    GGX,
    None,
}

/// Type of GPU query.
#[napi]
#[derive(Debug, Clone)]
pub enum GpuQueryType {
    Occlusion,
    Timestamp,
    TransformFeedbackPrimitives,
}

/// Compression texture type for texture compression.
#[napi]
#[derive(Debug, Clone)]
pub enum CompressionTextureType {
    ETC2,
    ETC2EAC,
    ASTC,
    BPTC,
    S3TC,
    S3TCSRGB,
    SVGA,
}

/// Shader type enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum ShaderType {
    Vertex = 0x8B31,                 // GL_VERTEX_SHADER
    Fragment = 0x8B30,               // GL_FRAGMENT_SHADER
    Geometry = 0x8DD9,               // GL_GEOMETRY_SHADER
    Compute = 0x91B9,                // GL_COMPUTE_SHADER
    TessellationControl = 0x8E87,    // GL_TESS_CONTROL_SHADER
    TessellationEvaluation = 0x8E88, // GL_TESS_EVALUATION_SHADER
}

/// Comparison function for depth testing and stencil operations.
#[napi]
#[derive(Debug, Clone)]
pub enum Comparison {
    Never = 0x0200,          // GL_NEVER
    Less = 0x0201,           // GL_LESS
    Equal = 0x0202,          // GL_EQUAL
    LessOrEqual = 0x0203,    // GL_LEQUAL
    Greater = 0x0204,        // GL_GREATER
    NotEqual = 0x0205,       // GL_NOTEQUAL
    GreaterOrEqual = 0x0206, // GL_GEQUAL
    Always = 0x0207,         // GL_ALWAYS
}

/// Blend equation type for blending operations.
#[napi]
#[derive(Debug, Clone)]
pub enum BlendEquation {
    Add = 0x8006,             // GL_FUNC_ADD
    Subtract = 0x800A,        // GL_FUNC_SUBTRACT
    ReverseSubtract = 0x800B, // GL_FUNC_REVERSE_SUBTRACT
    Min = 0x8007,             // GL_MIN
    Max = 0x8008,             // GL_MAX
}

/// Blend factor type for source and destination blending.
#[napi]
#[derive(Debug, Clone)]
pub enum BlendMultiplier {
    Zero = 0x0000,              // GL_ZERO
    One = 0x0001,               // GL_ONE
    SrcColor = 0x0300,          // GL_SRC_COLOR
    OneMinusSrcColor = 0x0301,  // GL_ONE_MINUS_SRC_COLOR
    DstColor = 0x0306,          // GL_DST_COLOR
    OneMinusDstColor = 0x0307,  // GL_ONE_MINUS_DST_COLOR
    SrcAlpha = 0x0302,          // GL_SRC_ALPHA
    OneMinusSrcAlpha = 0x0303,  // GL_ONE_MINUS_SRC_ALPHA
    DstAlpha = 0x0304,          // GL_DST_ALPHA
    OneMinusDstAlpha = 0x0305,  // GL_ONE_MINUS_DST_ALPHA
    Src1Color = 0x0308,         // GL_SRC1_COLOR
    OneMinusSrc1Color = 0x0309, // GL_ONE_MINUS_SRC1_COLOR
    Src1Alpha = 0x881A,         // GL_SRC1_ALPHA
    OneMinusSrc1Alpha = 0x881B, // GL_ONE_MINUS_SRC1_ALPHA
}

/// Cull face type for back-face culling.
#[napi]
#[derive(Debug, Clone)]
pub enum CullFace {
    None = 0,
    Front = 0x0404,        // GL_FRONT
    Back = 0x0405,         // GL_BACK
    FrontAndBack = 0x0408, // GL_FRONT_AND_BACK
}

/// Face winding order for front-face definition.
#[napi]
#[derive(Debug, Clone)]
pub enum FaceWinding {
    CounterClockwise = 0x0901, // GL_CCW
    Clockwise = 0x0900,        // GL_CW
}

/// Depth function for depth testing.
#[napi]
#[derive(Debug, Clone)]
pub enum DepthTest {
    Never = 0x0200,          // GL_NEVER
    Less = 0x0201,           // GL_LESS
    Equal = 0x0202,          // GL_EQUAL
    LessOrEqual = 0x0203,    // GL_LEQUAL
    Greater = 0x0204,        // GL_GREATER
    NotEqual = 0x0205,       // GL_NOTEQUAL
    GreaterOrEqual = 0x0206, // GL_GEQUAL
    Always = 0x0207,         // GL_ALWAYS
}

/// Stencil operation for stencil buffer.
#[napi]
#[derive(Debug, Clone)]
pub enum StencilOperation {
    Keep = 0x1E00,          // GL_KEEP
    Zero = 0x0000,          // GL_ZERO
    Replace = 0x1E01,       // GL_REPLACE
    Increment = 0x1E02,     // GL_INCR
    Decrement = 0x1E03,     // GL_DECR
    IncrementWrap = 0x8507, // GL_INCR_WRAP
    DecrementWrap = 0x8508, // GL_DECR_WRAP
    Invert = 0x150A,        // GL_INVERT
}

/// Texture minification filter.
#[napi]
#[derive(Debug, Clone)]
pub enum TextureMinFilter {
    Nearest = 0x2600,              // GL_NEAREST
    Linear = 0x2601,               // GL_LINEAR
    NearestMipmapNearest = 0x2700, // GL_NEAREST_MIPMAP_NEAREST
    LinearMipmapNearest = 0x2701,  // GL_LINEAR_MIPMAP_NEAREST
    NearestMipmapLinear = 0x2702,  // GL_NEAREST_MIPMAP_LINEAR
    LinearMipmapLinear = 0x2703,   // GL_LINEAR_MIPMAP_LINEAR
}

/// Texture magnification filter.
#[napi]
#[derive(Debug, Clone)]
pub enum TextureMagFilter {
    Nearest = 0x2600, // GL_NEAREST
    Linear = 0x2601,  // GL_LINEAR
}

/// Texture wrapping mode.
#[napi]
#[derive(Debug, Clone)]
pub enum TextureWrap {
    Repeat = 0x2901,         // GL_REPEAT
    MirroredRepeat = 0x8370, // GL_MIRRORED_REPEAT
    ClampToEdge = 0x812F,    // GL_CLAMP_TO_EDGE
    ClampToBorder = 0x812D,  // GL_CLAMP_TO_BORDER
}

/// Cube map side for texture cube maps.
#[napi]
#[derive(Debug, Clone)]
pub enum CubeMapSide {
    PositiveX = 0x8515, // GL_TEXTURE_CUBE_MAP_POSITIVE_X
    NegativeX = 0x8516, // GL_TEXTURE_CUBE_MAP_NEGATIVE_X
    PositiveY = 0x8517, // GL_TEXTURE_CUBE_MAP_POSITIVE_Y
    NegativeY = 0x8518, // GL_TEXTURE_CUBE_MAP_NEGATIVE_Y
    PositiveZ = 0x8519, // GL_TEXTURE_CUBE_MAP_POSITIVE_Z
    NegativeZ = 0x851A, // GL_TEXTURE_CUBE_MAP_NEGATIVE_Z
}

/// Texture internal format.
#[napi]
#[derive(Debug, Clone)]
pub enum TextureFormat {
    R8 = 0x8229,          // GL_R8
    R8I = 0x8231,         // GL_R8I
    R8UI = 0x8232,        // GL_R8UI
    R16F = 0x822D,        // GL_R16F
    R16I = 0x8233,        // GL_R16I
    R16UI = 0x8234,       // GL_R16UI
    R32F = 0x822E,        // GL_R32F
    R32I = 0x8235,        // GL_R32I
    R32UI = 0x8236,       // GL_R32UI
    R8G8 = 0x822B,        // GL_R8G8
    R8G8I = 0x8237,       // GL_R8G8I
    R8G8UI = 0x8238,      // GL_R8G8UI
    R8G8B8 = 0x1907,      // GL_RGB
    R8G8B8I = 0x8D8F,     // GL_RGB8I
    R8G8B8UI = 0x8D7D,    // GL_RGB8UI
    R8G8B8A8 = 0x1908,    // GL_RGBA
    R8G8B8A8I = 0x8D95,   // GL_RGBA8I
    R8G8B8A8UI = 0x8D81,  // GL_RGBA8UI
    R16F16I = 0x822F,     // GL_R16F (special 16-bit float)
    R16I16UI = 0x8239,    // GL_R16I (special 16-bit integer)
    R32F32I = 0x8230,     // GL_R32F (special 32-bit float)
    R32I32UI = 0x823A,    // GL_R32I (special 32-bit integer)
    R16F16I16UI = 0x881E, // GL_R16F (special for RGB)
    R16I16UI16F = 0x8E59, // GL_R16I (special for RGB)
    // R32F32I32UI = 0x8236,      // GL_R32UI (corrected from conflicting 0x8231)
    R16F16I16UI16F = 0x8E5A,   // GL_R16F (special for RGBA)
    R16I16UI16F16F = 0x8E5B,   // GL_R16I (special for RGBA)
    R32F32I32UI32F = 0x8E5C,   // GL_R32F (special for RGBA)
    R8G8B8A8Unorm = 0x8058,    // Special RGBA8
    R8G8B8A8Snorm = 0x8F97,    // Special RGBA8 SNORM
    R8G8B8A8Sint = 0x8D94,     // Special RGBA8 SINT
    R8G8B8A8Uint = 0x8D7C,     // Special RGBA8 UINT
    Depth16 = 0x81A5,          // GL_DEPTH_COMPONENT16
    Depth24 = 0x81A6,          // GL_DEPTH_COMPONENT24
    Depth32F = 0x8CAC,         // GL_DEPTH_COMPONENT32F
    Depth24Stencil8 = 0x88F0,  // GL_DEPTH24_STENCIL8
    Depth32FStencil8 = 0x8CAD, // GL_DEPTH32F_STENCIL8
    StencilIndex8 = 0x1901,    // GL_STENCIL_INDEX8
}

/// Data type for vertex attributes or uniform data.
#[napi]
#[derive(Debug, Clone)]
pub enum DataType {
    Byte = 0x1400,          // GL_BYTE
    UnsignedByte = 0x1401,  // GL_UNSIGNED_BYTE
    Short = 0x1402,         // GL_SHORT
    UnsignedShort = 0x1403, // GL_UNSIGNED_SHORT
    Int = 0x1404,           // GL_INT
    UnsignedInt = 0x1405,   // GL_UNSIGNED_INT
    HalfFloat = 0x140B,     // GL_HALF_FLOAT
    Float = 0x1406,         // GL_FLOAT
    Double = 0x140A,        // GL_DOUBLE
}

/// Primitive rendering type.
#[napi]
#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Points = 0x0000,                 // GL_POINTS
    Lines = 0x0001,                  // GL_LINES
    LineLoop = 0x0002,               // GL_LINE_LOOP
    LineStrip = 0x0003,              // GL_LINE_STRIP
    Triangles = 0x0004,              // GL_TRIANGLES
    TriangleStrip = 0x0005,          // GL_TRIANGLE_STRIP
    TriangleFan = 0x0006,            // GL_TRIANGLE_FAN
    LinesAdjacency = 0x000A,         // GL_LINES_ADJACENCY
    LineStripAdjacency = 0x000B,     // GL_LINE_STRIP_ADJACENCY
    TrianglesAdjacency = 0x000C,     // GL_TRIANGLES_ADJACENCY
    TriangleStripAdjacency = 0x000D, // GL_TRIANGLE_STRIP_ADJACENCY
    Patches = 0x000E,                // GL_PATCHES
}

/// Draw mode for rendering.
#[napi]
#[derive(Debug, Clone)]
pub enum DrawMode {
    Static = 0x88E4,  // GL_STATIC_DRAW
    Dynamic = 0x88E8, // GL_DYNAMIC_DRAW
    Stream = 0x88E0,  // GL_STREAM_DRAW
}

/// Transparency blending mode.
#[napi]
#[derive(Debug, Clone)]
pub enum Transparency {
    Opaque,
    Alpha,
    Additive,
    Multiply,
}

/// Filter mode for post-processing effects.
#[napi]
#[derive(Debug, Clone)]
pub enum FilterMode {
    Point,
    Linear,
    Gaussian,
}

/// Tone mapping algorithm for HDR rendering.
#[napi]
#[derive(Debug, Clone)]
pub enum ToneMapping {
    None,
    Reinhard,
    ACESFilmic,
    Filmic,
    Lottes,
    Uchimura,
    Unreal,
}

/// Background rendering mode.
#[napi]
#[derive(Debug, Clone)]
pub enum BackgroundMode {
    Color,
    CubeMap,
    Skybox,
    Environment,
    None,
}

/// Geometry type for rendering.
#[napi]
#[derive(Debug, Clone)]
pub enum GeometryType {
    Points,
    Lines,
    Triangles,
    Fan,
    Strip,
    Quads,
}

/// Material shading model type.
#[napi]
#[derive(Debug, Clone)]
pub enum MaterialShadingModel {
    PBR,
    Phong,
    Lambert,
    Toon,
    Unlit,
}

/// Light type enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum LightType {
    Directional,
    Point,
    Spot,
    Ambient,
}

/// Eye stereo mode for stereoscopic rendering.
#[napi]
#[derive(Debug, Clone)]
pub enum StereoMode {
    Mono,
    Left,
    Right,
    SideBySide,
    TopBottom,
}

/// Framebuffer attachment type.
#[napi]
#[derive(Debug, Clone)]
pub enum FramebufferAttachment {
    Color(Option<u32>),
    Depth,
    Stencil,
    DepthStencil,
}

/// Query result state.
#[napi]
#[derive(Debug, Clone)]
pub enum QueryResult {
    Waiting,
    Available,
    Unavailable,
}

/// Buffer mapping access type.
#[napi]
#[derive(Debug, Clone)]
pub enum MapAccess {
    Read,
    Write,
    ReadWrite,
    ReadWritePersistent,
}

/// Buffer fencing mode.
#[napi]
#[derive(Debug, Clone)]
pub enum FenceStatus {
    Signaled,
    Unsignaled,
    ConditionSatisfied,
    AlreadySignaled,
    TimeoutExpired,
    NotSignaled,
}

/// Buffer storage flag.
#[napi]
#[derive(Debug, Clone)]
pub enum BufferStorage {
    Read,
    Write,
    ReadWrite,
    DynamicStorage,
    ClientStorage,
    MapCoherent,
    MapPersistent,
    MapRead,
    MapWrite,
}

/// Texture array layer type.
#[napi]
#[derive(Debug, Clone)]
pub enum TextureArrayLayer {
    Array,
    ArrayBuffer,
    Texture2DMultisampleArray,
    ArrayBufferShareBuffer,
}

/// Framebuffer clear flag.
#[napi]
#[derive(Debug, Clone)]
pub enum ClearFlag {
    Color,
    Depth,
    Stencil,
    ColorDepth,
    ColorStencil,
    DepthStencil,
    All,
}

/// Tessellation primitive mode.
#[napi]
#[derive(Debug, Clone)]
pub enum TessellationMode {
    Triangles,
    Quads,
    Isolines,
}

/// Viewport scaling mode.
#[napi]
#[derive(Debug, Clone)]
pub enum ViewportScaling {
    Stretch,
    PreserveAspect,
    IntegerScale,
    Fit,
}

/// Buffer binding point.
#[napi]
#[derive(Debug, Clone)]
pub enum BufferBindingPoint {
    Vertex,
    Element,
    Uniform,
    TransformFeedback,
    Patch,
    Constant,
}

/// Execute mode for shader programs.
#[napi]
#[derive(Debug, Clone)]
pub enum ExecutionMode {
    Threaded,
    Immediate,
    Deferred,
    Async,
}

/// Core errors for the graphics system.
#[napi]
#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    /// General error with description
    General(String),
    /// Shader compilation failed
    ShaderCompilation(String),
    /// Program linking failed
    ProgramLinking(String),
    /// Texture creation failed
    TextureCreation(String),
    /// Buffer creation failed
    BufferCreation(String),
    /// Invalid operation
    InvalidOperation(String),
    /// Out of memory
    OutOfMemory(String),
    /// Invalid parameter
    InvalidParameter(String),
    /// Feature not supported
    FeatureNotSupported(String),
}

/// Renderer error enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum RendererError {
    /// Initialization failed
    InitializationFailed(String),
    /// Rendering failed
    RenderingFailed(String),
    /// Resource not found
    ResourceNotFound(String),
    /// Shader compilation failed
    ShaderCompilation(String),
    /// Internal error
    InternalError(String),
}

/// Render state error enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum RenderStateError {
    /// Invalid state combination
    InvalidCombination(String),
    /// State constraint violated
    ConstraintViolation(String),
    /// State not reset
    NotReset(String),
}

/// Debug message type.
#[napi]
#[derive(Debug, Clone)]
pub enum DebugType {
    Error,
    Deprecated,
    Undefined,
    Performance,
    Other,
    Marker,
    PushGroup,
    PopGroup,
}

/// Debug message parameter.
#[napi]
#[derive(Debug, Clone)]
pub enum DebugMessageParameter {
    Source,
    Type,
    Severity,
    MessageId,
}

/// Debug message.
#[napi]
#[derive(Debug, Clone)]
pub enum DebugMessage {
    Api,
    ShaderCompiler,
    WindowSystem,
    ThirdParty,
    Application,
    Other,
}

/// Hardware acceleration mode.
#[napi]
#[derive(Debug, Clone)]
pub enum HardwareAcceleration {
    Hardware,
    Software,
    Automatic,
}

/// Headless error enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum HeadlessError {
    /// Graphics context not available
    NoContext,
    /// Rendering to offscreen failed
    OffscreenFailed(String),
    /// Frame capture failed
    FrameCaptureFailed(String),
    /// Parameter validation failed
    InvalidParameter(String),
}

/// Window error enumeration.
#[napi]
#[derive(Debug, Clone)]
pub enum WindowError {
    /// Window creation failed
    CreationFailed(String),
    /// Window already exists
    AlreadyExists,
    /// Window not found
    NotFound,
    /// Invalid parameter
    InvalidParameter(String),
}

/// Render states error.
#[napi]
#[derive(Debug, Clone)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

/// Draw mode hint.
#[napi]
#[derive(Debug, Clone)]
pub enum DrawModeHint {
    Static,
    Dynamic,
    Stream,
}

/// Cull face enum (for compatibility with core module).
#[napi]
#[derive(Debug, Clone)]
pub enum Cull {
    None,
    Back,
    Front,
    FrontAndBack,
}

/// Buffer usage hint (type alias to BufferUsage for compatibility).
#[napi]
pub type BufferUsageHint = BufferUsage;

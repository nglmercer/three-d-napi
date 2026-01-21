//@ŧs-ignore
import * as three_d from "../index";
//@ts-ignore
import { Vector2,Scene,Vector3,Vector4,Matrix2,Matrix3,Point2,Point3,NDeg,NQuaternion,LightType,PixelFormat,TextureFilter,TextureWrap } from "../index";
const Deg = NDeg;
const Quaternion = NQuaternion;
console.log("\n=== three-d N-API Bindings Example ===\n");
// ============================================================================
// 1. Module Initialization & Basic Functions
// ============================================================================
console.log("1. Module Functions:");
console.log("   helloThreeD():", three_d.helloThreeD());
console.log("   get_version():", three_d.getVersion());

// ============================================================================
// 2. Context Management
// ============================================================================
console.log("\n2. Context Management:");
const Context = three_d.Context;
const ctx = new Context();

console.log("   Context created:", ctx.isValid());
console.log("   Context info:", ctx.getInfo());

const version = ctx.getVersion();
if (version) {
  console.log("   GL Version:", version.toString());
  console.log("   GLSL Target:", version.glslTarget());
  console.log("   Supports OpenGL 3.2?", version.supports(3, 2));
}

console.log("   GLSL Version:", ctx.getGlslVersion());

// ============================================================================
// 3. Camera Operations (Flat Coordinates)
// ============================================================================
console.log("\n3. Camera System:");
const Camera = three_d.Camera;

// Create with flat coordinates (no nested Point3/Vector3 objects)
const camera = new Camera(
  0,
  0,
  5, // position x, y, z
  0,
  0,
  0, // target x, y, z
  0,
  1,
  0, // up x, y, z
  60, // FOV degrees
  0.1, // near
  1000, // far
);

const camPos = camera.getPosition();
console.log("   Camera at position:", camPos);
console.log(
  "   Position coordinates:",
  `x=${camPos[0]}, y=${camPos[1]}, z=${camPos[2]}`,
);

// ============================================================================
// 4. Scene Camera (Scene-level camera management)
// ============================================================================
console.log("\n4. Scene Camera Management:");
const SceneCamera = three_d.Camera;
const Viewport = three_d.Viewport;

const viewport = new Viewport(0, 0, 800, 600);


// ============================================================================
// 5. Viewport Management
// ============================================================================
console.log("\n5. Viewport Management:");
const vp = new Viewport(100, 100, 400, 300);
console.log("   Viewport:", vp.getInfo());
console.log("   Aspect ratio:", vp.aspectRatio());
console.log("   Contains (200, 200)?", vp.contains(200, 200));

const originVp = Viewport.atOrigin(1920, 1080);
console.log("   Origin viewport:", originVp.getInfo());

// ============================================================================
// 6. Buffer Operations
// ============================================================================
console.log("\n6. Buffer System:");
const VertexBuffer = three_d.VertexBuffer;
const ElementBuffer = three_d.ElementBuffer;
const InstanceBuffer = three_d.InstanceBuffer;
const UniformBuffer = three_d.UniformBuffer;

// Vertex Buffer
const vbo = new VertexBuffer("positions", 1000, "static_draw");
console.log("   Vertex Buffer:", vbo.getInfo());
console.log("   Buffer Type:", vbo.getBufferType());
console.log("   Estimated Size:", vbo.estimateSizeBytes(), "bytes");
console.log("   Valid?", vbo.isValid());

// Element Buffer
const ebo = new ElementBuffer("indices", 3000, "dynamic_draw");
console.log("\n   Element Buffer:", ebo.getInfo());
console.log("   Buffer Type:", ebo.getBufferType());
console.log("   Estimated Size:", ebo.estimateSizeBytes(), "bytes");
console.log("   Usage:", ebo.getUsage());

// Instance Buffer
const instanceData = [];
for (let i = 0; i < 16 * 100; i++) {
  instanceData.push(Math.random() * 2.0 - 1.0); // Random transform data
}
const ibo = new InstanceBuffer("instances", 100, instanceData);
console.log("\n   Instance Buffer:", ibo.getInfo());
console.log("   Buffer Type:", ibo.getBufferType());
console.log("   Estimated Size:", ibo.estimateSizeBytes(), "bytes");
console.log("   Valid?", ibo.isValid());

// Uniform Buffer
const ubo = new UniformBuffer("matrices", 0, 1024);
console.log("\n   Uniform Buffer:", ubo.getInfo());
console.log("   Buffer Type:", ubo.getBufferType());
console.log("   Size:", ubo.getSizeBytes(), "bytes");
console.log("   Valid?", ubo.isValid());

// ============================================================================
// 7. Math Types (Prelude)
// ============================================================================
console.log("\n7. Math Types (Prelude):");

// Points

const p2d = new Point2(1.5, 2.5);
console.log("   Point2:", `(${p2d.x}, ${p2d.y})`);

const p3d = new Point3(1.0, 2.0, 3.0);
console.log("   Point3:", `(${p3d.x}, ${p3d.y}, ${p3d.z})`);


const v2 = new Vector2(3.0, 4.0);
const v2Len = v2.length();
const v2Norm = v2.normalize();
console.log("   Vector2:", `(${v2.x}, ${v2.y})`);
console.log("   Vector2 Length:", v2Len);
console.log(
  "   Vector2 Normalized:",
  `(${v2Norm.x.toFixed(3)}, ${v2Norm.y.toFixed(3)})`,
);

const v3 = new Vector3(1.0, 2.0, 3.0);
console.log("   Vector3:", `(${v3.x}, ${v3.y}, ${v3.z})`);

const v4 = new Vector4(1.0, 2.0, 3.0, 4.0);
console.log("   Vector4:", `(${v4.x}, ${v4.y}, ${v4.z}, ${v4.w})`);

const m2 = new Matrix2();
console.log("   Matrix2 (identity):", m2.data);
const m2_id = Matrix2.identity();
console.log("   Matrix2.id:", m2_id.data);

const m3 = Matrix3.identity();
console.log("   Matrix3 (identity):", m3.data);

const m4 = three_d.Matrix4.identity();
console.log("   Matrix4 (identity):", m4.data);

// Quaternions & Angles


const quat = new Quaternion(0, 1, 0, 0.5); // w, x, y, z
console.log(
  "   Quaternion:",
  `w=${quat.w}, x=${quat.x}, y=${quat.y}, z=${quat.z}`,
);

const angle = new NDeg(45.0);
const deg = new Deg(90.0); // NDeg is also exported as Deg
console.log("   Angle in degrees:", angle.value);

// ============================================================================
// 8. Scene Management
// ============================================================================
console.log("\n8. Scene Management:");
const SceneObject = three_d.SceneObject;
const LightSource = three_d.LightSource;

// Scene
const scene = new Scene("MyScene", 10, 20, 30, 255);
console.log("   Scene Name:", scene.name);
console.log(
  "   Background Color:",
  `r=${scene.bgR}, g=${scene.bgB}, b=${scene.bgB}, a=${scene.bgA}`,
);

// Scene Object
const obj = new SceneObject(1.0, 2.0, 3.0, 1.5, "Cube");
console.log("\n   Scene Object:", obj.name);
console.log(
  "   Position:",
  `x=${obj.positionX}, y=${obj.positionY}, z=${obj.positionZ}`,
);
console.log("   Scale:", obj.scale);

// Lights
const light1 = new LightSource(
  LightType.Directional,
  0,
  10,
  5,
  255,
  255,
  255,
  1.5,
);
console.log("\n   Directional Light:", light1.lightType);
console.log(
  "   Position:",
  `x=${light1.posX}, y=${light1.posY}, z=${light1.posZ}`,
);
console.log(
  "   Color:",
  `r=${light1.colorR}, g=${light1.colorG}, b=${light1.colorB}`,
);
console.log("   Intensity:", light1.intensity);

const light2 = new LightSource(LightType.Point, 5, 2, 0, 255, 128, 64, 2.0);
console.log("\n   Point Light:", light2.lightType);
console.log(
  "   Position:",
  `x=${light2.posX}, y=${light2.posY}, z=${light2.posZ}`,
);

// ============================================================================
// 9. Texture System
// ============================================================================
console.log("\n9. Texture System:");
const Texture2D = three_d.Texture2D;
const Texture2DArray = three_d.Texture2DArray;
const TextureCube = three_d.TextureCube;


// 2D Texture
const tex2d = new Texture2D(
  "diffuse_map",
  512,
  512,
  PixelFormat.Rgba8,
  TextureFilter.LinearMipmapLinear,
  TextureFilter.Linear,
  TextureWrap.Repeat,
  TextureWrap.Repeat,
);
console.log("   Texture2D:", tex2d.getInfo());

// Texture Array
const texArray = new Texture2DArray(
  "material_array",
  256,
  256,
  10,
  PixelFormat.Rgba8,
  TextureFilter.Linear,
  TextureFilter.Linear,
);
console.log("   Texture2DArray:", texArray.getInfo());

// Cube Map
const texCube = new TextureCube(
  "environment_map",
  1024,
  PixelFormat.Rgba8,
  TextureFilter.LinearMipmapLinear,
  TextureFilter.Linear,
  TextureWrap.ClampToEdge,
  TextureWrap.ClampToEdge,
);
console.log("   TextureCube:", texCube.getInfo());
console.log(
  "   TextureCube Estimated Size:",
  texCube.estimateSizeBytes(),
  "bytes",
);

// ============================================================================
// 10. Bounding Volume
// ============================================================================
console.log("\n10. Bounding Volumes:");
const AABB = three_d.AxisAlignedBoundingBox;

const min = new Point3(-10, -10, -10);
const max = new Point3(10, 10, 10);
const bounds = new AABB(min.x, min.y, min.z, max.x, max.y, max.z);

console.log("   AABB created from min/max");
console.log("   Center:", bounds.center());
console.log("   Size:", bounds.size());
console.log("   Contains (0,0,0)?", bounds.contains(0, 0, 0));
console.log("   Contains (11,11,11)?", bounds.contains(11, 11, 11));

// Merge with another
const expanded = new AABB(-20, -20, -20, 20, 20, 20);
const merged = bounds.merge(expanded);
console.log("   Merged Center:", merged.center());
console.log("   Merged Size:", merged.size());

// ============================================================================
// 11. Renderer Wrapper
// ============================================================================
console.log("\n11. Renderer:");
const Renderer = three_d.Renderer;

const renderer = new Renderer(1920, 1080, "Three-D N-API Demo");
console.log("   Renderer Info:", renderer.getInfo());
console.log("   Initializing...");
renderer.init();
console.log("   Init complete:", renderer.isInitialized);
console.log("   Frame render result:", renderer.renderFrame());

// ============================================================================
// 12. Context Native Types (Native Handles)
// ============================================================================
console.log("\n12. Native GPU Handles (Mock):");
const NativeBuffer = three_d.NativeBuffer;
const NativeFence = three_d.NativeFence;
const NativeTexture = three_d.NativeTexture;
const NativeProgram = three_d.NativeProgram;

const nBuffer = new NativeBuffer(1001, 0x8892, 1024);
console.log("   NativeBuffer:", nBuffer.getInfo());

const nFence = new NativeFence(99999, false);
console.log("   NativeFence:", nFence.getInfo());

const nTexture = new NativeTexture(2001, 0x0de1, 512, 512, 0x8058);
console.log("   NativeTexture:", nTexture.getInfo());

const nProgram = new NativeProgram(3001, true, 12);
console.log("   NativeProgram:", nProgram.getInfo());

// ============================================================================
// 13. Active Info Wrappers
// ============================================================================
console.log("\n13. Active Info Wrappers:");
const ActiveAttribute = three_d.ActiveAttribute;
const ActiveUniform = three_d.ActiveUniform;
const ActiveTransformFeedback = three_d.ActiveTransformFeedback;

const activeAttr = new ActiveAttribute("position", 0x1406, 3); // GL_FLOAT
console.log("   ActiveAttribute:", activeAttr.getInfo());

const activeUniform = new ActiveUniform("modelMatrix", 0x8b5d, 1); // GL_FLOAT_MAT4
console.log("   ActiveUniform:", activeUniform.getInfo());

const activeTF = new ActiveTransformFeedback("tf_out", 0, 4);
console.log("   ActiveTransformFeedback:", activeTF.getInfo());

// ============================================================================
// 14. Program & Shader Wrappers
// ============================================================================
console.log("\n14. Shader & Program Wrappers:");
const NativeShader = three_d.NativeShader;
const Program = three_d.Program;

const shader = new NativeShader(4001, 0x8b30, 150); // GL_VERTEX_SHADER
console.log("   NativeShader:", shader.getInfo());

const program = new Program(ctx, "vertex_source", "fragment_source");
console.log("   Program created with context reference",program);

// ============================================================================
// 15. Debug & Binary Wrappers
// ============================================================================
console.log("\n15. Debug & Binary Wrappers:");
const DebugMessageLogEntry = three_d.DebugMessageLogEntry;
const ProgramBinary = three_d.ProgramBinary;

const debugMsg = new DebugMessageLogEntry(
  0x824c,
  0x824d,
  1,
  0x91e3,
  "Shader compilation successful",
);
console.log("   DebugMessage:", debugMsg.getInfo());
console.log(
  "   JS Test:",
  debugMsg.source,
  debugMsg.messageType,
  debugMsg.message,
);

const pBinary = new ProgramBinary([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0x8740);
console.log("   ProgramBinary:", pBinary.getInfo());
console.log("   Binary length:", pBinary.length());

// ============================================================================
// 16. Version & Native Wrappers
// ============================================================================
console.log("\n16. Version Wrappers:");
const Version = three_d.Version;

const v32 = new Version(3, 2);
console.log("   Version (3,2):", v32.toString());
console.log("   GLSL Target:", v32.glslTarget());
console.log("   Supports 4.6?", v32.supports(4, 6));
console.log("   Supports 3.2?", v32.supports(3, 2));


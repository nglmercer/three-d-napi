import { Renderer, Camera, Vector3, Matrix4, NSrgba } from '../index.js';

async function main() {
  console.log('🚀 Initializing Three-D NAPI example...');

  try {
    // 1. Create a Renderer
    // Parameters: width, height, title
    const renderer = new Renderer(1280, 720, "Three-D NAPI Advanced Demo");
    renderer.init();
    console.log('✅ Renderer initialized');
    console.log('ℹ️ Renderer info:', renderer.getInfo());

    // 2. Setup Camera
    // Parameters: position(x,y,z), target(x,y,z), up(x,y,z), fovDegrees, near, far
    const camera = new Camera(
      5.0, 5.0, 5.0,  // position
      0.0, 0.0, 0.0,  // target
      0.0, 1.0, 0.0,  // up
      45.0,           // fov
      0.1,            // near
      1000.0          // far
    );
    console.log('📸 Camera positioned at:', camera.getPosition());

    // 3. Vector math demonstrations
    const v1 = new Vector3(1, 0, 0);
    const v2 = new Vector3(0, 1, 0);
    const v3 = v1.cross(v2); // Should be (0, 0, 1)
    
    console.log('🔢 Vector Math:');
    console.log(`   v1: [${v1.x}, ${v1.y}, ${v1.z}]`);
    console.log(`   v2: [${v2.x}, ${v2.y}, ${v2.z}]`);
    console.log(`   v1 x v2 = [${v3.x}, ${v3.y}, ${v3.z}]`);
    console.log(`   v3 length: ${v3.length()}`);
    console.log(`   v3 normalized:`, v3.normalize());

    // 4. Matrix operations
    const identity = Matrix4.identity();
    console.log('📐 Identity Matrix loaded (first 4 elements):', identity.data.slice(0, 4));

    // 5. Color configuration
    const orange = new NSrgba(1.0, 0.5, 0.0, 1.0);
    console.log('🎨 Sample Color (Orange):', `rgba(${orange.r * 255}, ${orange.g * 255}, ${orange.b * 255}, ${orange.a})`);

    // 6. Simulate rendering a frame
    const frameSuccess = renderer.renderFrame();
    console.log({frameSuccess});
    const timeout = new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 10000);
    });
    await timeout;
  } catch (error) {
    console.error('❌ Error in example:', error);
  }
}

main().catch(err => {
  console.error('💥 Fatal error:', err);
  process.exit(1);
});

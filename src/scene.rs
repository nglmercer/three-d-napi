use napi_derive::napi;

/// Wrapper for internal content types (texture formats, etc.)
#[napi]
#[derive(Debug, Clone)]
pub enum PixelFormat {
    Rgba8,
    Rgb8,
    R8,
    Depth24,
    Depth32F,
    Rg16F,
    Rgba16F,
    Rgba32F,
}

/// Loading/filter hint for textures.
#[napi]
#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

/// Wrapping mode for textures.
#[napi]
#[derive(Debug, Clone, Copy)]
pub enum TextureWrap {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

/// Represents a 3D point in space.
/// Note: We use individual fields (x, y, z) instead of a nested Point3 struct
/// to avoid complex FromNapiValue/ToNapiValue derivation issues with nested structs in N-API.
#[napi]
#[derive(Debug, Clone)]
pub struct SceneObject {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub scale: f64,
    pub name: Option<String>,
}

#[napi]
impl SceneObject {
    #[napi(constructor)]
    pub fn new(
        position_x: f64,
        position_y: f64,
        position_z: f64,
        scale: Option<f64>,
        name: Option<String>,
    ) -> Self {
        SceneObject {
            position_x,
            position_y,
            position_z,
            scale: scale.unwrap_or(1.0),
            name,
        }
    }
}

/// Represents a light source type.
#[napi]
#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Point,
    Directional,
    Ambient,
}

/// Represents a light source.
/// Uses flat fields for N-API compatibility.
#[napi]
#[derive(Debug, Clone)]
pub struct LightSource {
    pub light_type: LightType,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub intensity: f64,
}

#[napi]
impl LightSource {
    #[napi(constructor)]
    pub fn new(
        light_type: LightType,
        pos_x: f64,
        pos_y: f64,
        pos_z: f64,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        intensity: f64,
    ) -> Self {
        LightSource {
            light_type,
            pos_x,
            pos_y,
            pos_z,
            color_r,
            color_g,
            color_b,
            intensity,
        }
    }
}

/// Represents the Scene.
/// Uses flat fields for N-API compatibility.
#[napi]
#[derive(Debug)]
pub struct Scene {
    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,
    pub bg_a: u8,
    pub name: Option<String>,
}

#[napi]
impl Scene {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        bg_r: Option<u8>,
        bg_g: Option<u8>,
        bg_b: Option<u8>,
        bg_a: Option<u8>,
    ) -> Self {
        Scene {
            name,
            bg_r: bg_r.unwrap_or(10),
            bg_g: bg_g.unwrap_or(10),
            bg_b: bg_b.unwrap_or(10),
            bg_a: bg_a.unwrap_or(255),
        }
    }
}

/// Represents a 2D Texture wrapper.
/// Used for materials, render targets, and sampling.
#[napi]
pub struct Texture2D {
    name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
}

#[napi]
impl Texture2D {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        width: u32,
        height: u32,
        format: PixelFormat,
        min_filter: Option<TextureFilter>,
        mag_filter: Option<TextureFilter>,
        wrap_s: Option<TextureWrap>,
        wrap_t: Option<TextureWrap>,
    ) -> Self {
        Texture2D {
            name,
            width,
            height,
            format,
            min_filter: min_filter.unwrap_or(TextureFilter::LinearMipmapLinear),
            mag_filter: mag_filter.unwrap_or(TextureFilter::Linear),
            wrap_s: wrap_s.unwrap_or(TextureWrap::Repeat),
            wrap_t: wrap_t.unwrap_or(TextureWrap::Repeat),
        }
    }

    /// Returns texture info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "2DTexture(name={:?}, {}x{}, format={:?})",
            self.name, self.width, self.height, self.format
        )
    }
}

/// Represents an array of 2D Textures.
/// Used for texture arrays (instanced rendering, decals, etc.).
#[napi]
pub struct Texture2DArray {
    name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
}

#[napi]
impl Texture2DArray {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        width: u32,
        height: u32,
        depth: u32,
        format: PixelFormat,
        min_filter: Option<TextureFilter>,
        mag_filter: Option<TextureFilter>,
        wrap_s: Option<TextureWrap>,
        wrap_t: Option<TextureWrap>,
    ) -> Self {
        Texture2DArray {
            name,
            width,
            height,
            depth,
            format,
            min_filter: min_filter.unwrap_or(TextureFilter::Linear),
            mag_filter: mag_filter.unwrap_or(TextureFilter::Linear),
            wrap_s: wrap_s.unwrap_or(TextureWrap::Repeat),
            wrap_t: wrap_t.unwrap_or(TextureWrap::Repeat),
        }
    }

    /// Returns texture array info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "2DTextureArray(name={:?}, {}x{}x{}, format={:?})",
            self.name, self.width, self.height, self.depth, self.format
        )
    }
}

/// Represents a Cube Map Texture wrapper.
/// Used for skyboxes, reflections, environment mapping.
/// Each side of the cube (6 faces) is a 2D texture of the same size.
#[napi]
pub struct TextureCube {
    name: Option<String>,
    pub size: u32,
    pub format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
}

#[napi]
impl TextureCube {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        size: u32,
        format: PixelFormat,
        min_filter: Option<TextureFilter>,
        mag_filter: Option<TextureFilter>,
        wrap_s: Option<TextureWrap>,
        wrap_t: Option<TextureWrap>,
    ) -> Self {
        TextureCube {
            name,
            size,
            format,
            min_filter: min_filter.unwrap_or(TextureFilter::LinearMipmapLinear),
            mag_filter: mag_filter.unwrap_or(TextureFilter::Linear),
            wrap_s: wrap_s.unwrap_or(TextureWrap::ClampToEdge),
            wrap_t: wrap_t.unwrap_or(TextureWrap::ClampToEdge),
        }
    }

    /// Returns texture cube info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "Cubemap(name={:?}, {}x{}, faces=6, format={:?})",
            self.name, self.size, self.size, self.format
        )
    }

    /// Returns the estimated memory usage in bytes.
    #[napi]
    pub fn estimate_size_bytes(&self) -> u64 {
        let components = match self.format {
            PixelFormat::Rgba8 => 4,
            PixelFormat::Rgb8 => 3,
            PixelFormat::R8 => 1,
            PixelFormat::Depth24 => 3,
            PixelFormat::Depth32F => 4,
            PixelFormat::Rg16F => 4,
            PixelFormat::Rgba16F => 8,
            PixelFormat::Rgba32F => 16,
        };
        (self.size as u64) * (self.size as u64) * components * 6
    }
}

/// Represents a 3D Texture wrapper.
/// Used for volumetric data, medical imaging, etc.
#[napi]
pub struct Texture3D {
    name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
    pub wrap_r: TextureWrap,
}

#[napi]
impl Texture3D {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        width: u32,
        height: u32,
        depth: u32,
        format: PixelFormat,
        min_filter: Option<TextureFilter>,
        mag_filter: Option<TextureFilter>,
        wrap_s: Option<TextureWrap>,
        wrap_t: Option<TextureWrap>,
        wrap_r: Option<TextureWrap>,
    ) -> Self {
        Texture3D {
            name,
            width,
            height,
            depth,
            format,
            min_filter: min_filter.unwrap_or(TextureFilter::Linear),
            mag_filter: mag_filter.unwrap_or(TextureFilter::Linear),
            wrap_s: wrap_s.unwrap_or(TextureWrap::ClampToEdge),
            wrap_t: wrap_t.unwrap_or(TextureWrap::ClampToEdge),
            wrap_r: wrap_r.unwrap_or(TextureWrap::ClampToEdge),
        }
    }

    /// Returns texture 3D info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "3DTexture(name={:?}, {}x{}x{}, format={:?})",
            self.name, self.width, self.height, self.depth, self.format
        )
    }
}

/// Represents a Cube Map Texture wrapper (alias for TextureCube that matches lib.md).
/// Used for skyboxes, reflections, environment mapping.
/// Each side of the cube (6 faces) is a 2D texture of the same size.
#[napi]
pub struct TextureCubeMap {
    name: Option<String>,
    pub size: u32,
    pub format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
}

#[napi]
impl TextureCubeMap {
    #[napi(constructor)]
    pub fn new(
        name: Option<String>,
        size: u32,
        format: PixelFormat,
        min_filter: Option<TextureFilter>,
        mag_filter: Option<TextureFilter>,
        wrap_s: Option<TextureWrap>,
        wrap_t: Option<TextureWrap>,
    ) -> Self {
        TextureCubeMap {
            name,
            size,
            format,
            min_filter: min_filter.unwrap_or(TextureFilter::LinearMipmapLinear),
            mag_filter: mag_filter.unwrap_or(TextureFilter::Linear),
            wrap_s: wrap_s.unwrap_or(TextureWrap::ClampToEdge),
            wrap_t: wrap_t.unwrap_or(TextureWrap::ClampToEdge),
        }
    }

    /// Returns cubemap info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "TextureCubeMap(name={:?}, {}x{}, faces=6, format={:?})",
            self.name, self.size, self.size, self.format
        )
    }

    /// Returns the estimated memory usage in bytes.
    #[napi]
    pub fn estimate_size_bytes(&self) -> u64 {
        let components = match self.format {
            PixelFormat::Rgba8 => 4,
            PixelFormat::Rgb8 => 3,
            PixelFormat::R8 => 1,
            PixelFormat::Depth24 => 3,
            PixelFormat::Depth32F => 4,
            PixelFormat::Rg16F => 4,
            PixelFormat::Rgba16F => 8,
            PixelFormat::Rgba32F => 16,
        };
        (self.size as u64) * (self.size as u64) * components * 6
    }
}

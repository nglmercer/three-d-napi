use napi_derive::napi;

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

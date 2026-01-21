use napi_derive::napi;
use three_d::{Deg, Matrix4, Point3, Quaternion, SquareMatrix, Vector3, Vector4};

/// Represents a 2D Point.
/// Note: `napi` supports `f64` better than `f32`, so we cast internally.
#[napi]
pub struct NPoint2 {
    pub x: f64,
    pub y: f64,
}

#[napi]
impl NPoint2 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        NPoint2 { x, y }
    }
}

/// Represents a 3D Point (Coordinate).
#[napi]
pub struct NPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[napi]
impl NPoint3 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        NPoint3 { x, y, z }
    }
}

impl From<&NPoint3> for Point3<f32> {
    fn from(p: &NPoint3) -> Self {
        Point3::new(p.x as f32, p.y as f32, p.z as f32)
    }
}

impl From<&Point3<f32>> for NPoint3 {
    fn from(p: &Point3<f32>) -> Self {
        NPoint3 {
            x: p.x as f64,
            y: p.y as f64,
            z: p.z as f64,
        }
    }
}

/// Represents a 3D Vector.
/// Used for direction, normal, etc.
#[napi]
pub struct NVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[napi]
impl NVector3 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        NVector3 { x, y, z }
    }
}

impl From<&NVector3> for Vector3<f32> {
    fn from(v: &NVector3) -> Self {
        Vector3::new(v.x as f32, v.y as f32, v.z as f32)
    }
}

impl From<&Vector3<f32>> for NVector3 {
    fn from(v: &Vector3<f32>) -> Self {
        NVector3 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}

/// Represents a 4x4 Matrix (Transformation).
/// Stored internally as f32 for three-d, exposed as f64 for JS.
#[napi]
pub struct NMatrix4 {
    // Stored as a flat array of 16 f64s (column-major)
    pub data: Vec<f64>,
}

#[napi]
impl NMatrix4 {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> Self {
        let d = data.unwrap_or_else(|| {
            // Return identity matrix by default
            vec![0.0; 16]
        });
        if d.len() != 16 {
            panic!("Matrix4 data must have 16 elements");
        }
        NMatrix4 { data: d }
    }

    #[napi]
    pub fn identity() -> Self {
        let mat = Matrix4::identity();
        NMatrix4::from_matrix4(&mat)
    }

    pub fn from_matrix4(mat: &Matrix4<f32>) -> Self {
        let mut data = Vec::with_capacity(16);
        for i in 0..4 {
            let col = mat[i];
            data.push(col.x as f64);
            data.push(col.y as f64);
            data.push(col.z as f64);
            data.push(col.w as f64);
        }
        NMatrix4 { data }
    }

    /// Returns the internal cgmath Matrix4 representation for use in three-d.
    pub fn to_matrix4(&self) -> Matrix4<f32> {
        // Convert f64 back to f32
        let data_f32: Vec<f32> = self.data.iter().map(|&x| x as f32).collect();
        let b: &[f32] = &data_f32;
        // cgmath Matrix4 is stored as columns (x, y, z, w)
        // Input data is assumed to be column-major
        Matrix4 {
            x: Vector4::new(b[0], b[1], b[2], b[3]),
            y: Vector4::new(b[4], b[5], b[6], b[7]),
            z: Vector4::new(b[8], b[9], b[10], b[11]),
            w: Vector4::new(b[12], b[13], b[14], b[15]),
        }
    }
}

/// Represents a Rotation Angle in Degrees.
#[napi]
pub struct NDeg {
    pub value: f64,
}

#[napi]
impl NDeg {
    #[napi(constructor)]
    pub fn new(value: f64) -> Self {
        NDeg { value }
    }
}

impl From<&NDeg> for Deg<f32> {
    fn from(d: &NDeg) -> Self {
        Deg(d.value as f32)
    }
}

/// Represents a Quaternion for rotation.
#[napi]
pub struct NQuaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[napi]
impl NQuaternion {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        NQuaternion { w, x, y, z }
    }
}

impl From<&NQuaternion> for Quaternion<f32> {
    fn from(q: &NQuaternion) -> Self {
        Quaternion::new(q.w as f32, q.x as f32, q.y as f32, q.z as f32)
    }
}

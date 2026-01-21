use napi_derive::napi;
use three_d;
use three_d::{
    Deg as ThreeDDeg, Matrix2 as ThreeDMatrix2, Matrix3 as ThreeDMatrix3, Matrix4 as ThreeDMatrix4,
    Point2 as ThreeDPoint2, Point3 as ThreeDPoint3, Quaternion as ThreeDQuaternion,
    Rad as ThreeDRad, SquareMatrix, Srgba as ThreeDSrgba, Vector2 as ThreeDVector2,
    Vector3 as ThreeDVector3, Vector4 as ThreeDVector4,
};

// Import N-API types (renamed to avoid conflicts with three_d types)
// Note: We need to use fully qualified paths to avoid creating circular dependencies

/// Represents a 2D Point.
/// Note: `napi` supports `f64` better than `f32`, so we cast internally.
#[napi]
#[derive(Debug, Clone)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

#[napi]
impl Point2 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Point2 { x, y }
    }
}

impl From<&Point2> for ThreeDPoint2<f32> {
    fn from(p: &Point2) -> Self {
        ThreeDPoint2::new(p.x as f32, p.y as f32)
    }
}

impl From<&ThreeDPoint2<f32>> for Point2 {
    fn from(p: &ThreeDPoint2<f32>) -> Self {
        Point2 {
            x: p.x as f64,
            y: p.y as f64,
        }
    }
}

/// Represents a 3D Point (Coordinate).
#[napi]
#[derive(Debug, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[napi]
impl Point3 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }
}

impl From<&Point3> for ThreeDPoint3<f32> {
    fn from(p: &Point3) -> Self {
        ThreeDPoint3::new(p.x as f32, p.y as f32, p.z as f32)
    }
}

impl From<&ThreeDPoint3<f32>> for Point3 {
    fn from(p: &ThreeDPoint3<f32>) -> Self {
        Point3 {
            x: p.x as f64,
            y: p.y as f64,
            z: p.z as f64,
        }
    }
}

/// Represents a 2D Vector.
/// Used for direction, normal, etc.
#[napi]
#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[napi]
impl Vector2 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    /// Returns the magnitude (length) of this vector.
    /// Calculate sqrt(x^2 + y^2)
    #[napi]
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Returns a normalized (unit length) version of this vector.
    /// Returns zero vector if length is zero.
    #[napi]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Vector2 { x: 0.0, y: 0.0 }
        } else {
            Vector2 {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    /// Returns the dot product of this vector and another vector.
    #[napi]
    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the perpendicular vector (90 degrees counterclockwise).
    /// (-y, x)
    #[napi]
    pub fn perp(&self) -> Self {
        Vector2 {
            x: -self.y,
            y: self.x,
        }
    }

    /// Returns the angle in radians between this vector and the positive X axis.
    #[napi]
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Returns the distance to another vector.
    #[napi]
    pub fn distance_to(&self, other: &Vector2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    /// Linearly interpolates between this vector and another vector by t.
    /// t=0 returns this vector, t=1 returns the other vector.
    #[napi]
    pub fn lerp(&self, other: &Vector2, t: f64) -> Self {
        Vector2 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Adds another vector to this vector and returns the result.
    #[napi]
    pub fn add(&self, other: &Vector2) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtracts another vector from this vector and returns the result.
    #[napi]
    pub fn sub(&self, other: &Vector2) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiplies this vector by a scalar and returns the result.
    #[napi]
    pub fn mul(&self, scalar: f64) -> Self {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Divides this vector by a scalar and returns the result.
    #[napi]
    pub fn div(&self, scalar: f64) -> Self {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl From<&Vector2> for ThreeDVector2<f32> {
    fn from(v: &Vector2) -> Self {
        ThreeDVector2::new(v.x as f32, v.y as f32)
    }
}

impl From<&ThreeDVector2<f32>> for Vector2 {
    fn from(v: &ThreeDVector2<f32>) -> Self {
        Vector2 {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}

/// Represents a 3D Vector.
/// Used for direction, normal, etc.
#[napi]
#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[napi]
impl Vector3 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    /// Returns the magnitude (length) of this vector.
    /// Calculate sqrt(x^2 + y^2 + z^2)
    #[napi]
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    /// Returns a normalized (unit length) version of this vector.
    /// Returns zero vector if length is zero.
    #[napi]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vector3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }

    /// Returns the dot product of this vector and another vector.
    #[napi]
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of this vector and another vector.
    #[napi]
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the distance to another vector.
    #[napi]
    pub fn distance_to(&self, other: &Vector3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
    }

    /// Linearly interpolates between this vector and another vector by t.
    /// t=0 returns this vector, t=1 returns the other vector.
    #[napi]
    pub fn lerp(&self, other: &Vector3, t: f64) -> Self {
        Vector3 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    /// Returns a vector with all components negated.
    #[napi]
    pub fn negate(&self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Adds another vector to this vector and returns the result.
    #[napi]
    pub fn add(&self, other: &Vector3) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtracts another vector from this vector and returns the result.
    #[napi]
    pub fn sub(&self, other: &Vector3) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Multiplies this vector by a scalar and returns the result.
    #[napi]
    pub fn mul(&self, scalar: f64) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Divides this vector by a scalar and returns the result.
    #[napi]
    pub fn div(&self, scalar: f64) -> Self {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl From<&Vector3> for ThreeDVector3<f32> {
    fn from(v: &Vector3) -> Self {
        ThreeDVector3::new(v.x as f32, v.y as f32, v.z as f32)
    }
}

impl From<&ThreeDVector3<f32>> for Vector3 {
    fn from(v: &ThreeDVector3<f32>) -> Self {
        Vector3 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}

/// Represents a 4D Vector (used for colors, quaternions, etc.).
#[napi]
#[derive(Debug, Clone)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[napi]
impl Vector4 {
    #[napi(constructor)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vector4 { x, y, z, w }
    }

    /// Returns the magnitude (length) of this vector.
    /// Calculate sqrt(x^2 + y^2 + z^2 + w^2)
    #[napi]
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    /// Returns a normalized (unit length) version of this vector.
    /// Returns zero vector if length is zero.
    #[napi]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }
        } else {
            Vector4 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        }
    }

    /// Returns the dot product of this vector and another vector.
    #[napi]
    pub fn dot(&self, other: &Vector4) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Returns the distance to another vector.
    #[napi]
    pub fn distance_to(&self, other: &Vector4) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        ((dx * dx) + (dy * dy) + (dz * dz) + (dw * dw)).sqrt()
    }

    /// Linearly interpolates between this vector and another vector by t.
    /// t=0 returns this vector, t=1 returns the other vector.
    #[napi]
    pub fn lerp(&self, other: &Vector4, t: f64) -> Self {
        Vector4 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            w: self.w + (other.w - self.w) * t,
        }
    }

    /// Returns a vector with all components negated.
    #[napi]
    pub fn negate(&self) -> Self {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    /// Adds another vector to this vector and returns the result.
    #[napi]
    pub fn add(&self, other: &Vector4) -> Self {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    /// Subtracts another vector from this vector and returns the result.
    #[napi]
    pub fn sub(&self, other: &Vector4) -> Self {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    /// Multiplies this vector by a scalar and returns the result.
    #[napi]
    pub fn mul(&self, scalar: f64) -> Self {
        Vector4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    /// Divides this vector by a scalar and returns the result.
    #[napi]
    pub fn div(&self, scalar: f64) -> Self {
        Vector4 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl From<&Vector4> for ThreeDVector4<f32> {
    fn from(v: &Vector4) -> Self {
        ThreeDVector4::new(v.x as f32, v.y as f32, v.z as f32, v.w as f32)
    }
}

impl From<&ThreeDVector4<f32>> for Vector4 {
    fn from(v: &ThreeDVector4<f32>) -> Self {
        Vector4 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
            w: v.w as f64,
        }
    }
}

/// Represents a 2x2 Matrix (stored column-major).
/// Stored internally as f32 for three-d, exposed as f64 for JS.
#[napi]
#[derive(Debug, Clone)]
pub struct Matrix2 {
    // Stored as a flat array of 4 f64s (column-major)
    pub data: Vec<f64>,
}

#[napi]
impl Matrix2 {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> Self {
        let d = data.unwrap_or_else(|| {
            // Return identity matrix by default: 2x2 identity is [[1,0],[0,1]]
            vec![1.0, 0.0, 0.0, 1.0]
        });
        if d.len() != 4 {
            panic!("Matrix2 data must have 4 elements");
        }
        Matrix2 { data: d }
    }

    #[napi]
    pub fn identity() -> Self {
        Matrix2 {
            data: vec![1.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn from_matrix2(mat: &ThreeDMatrix2<f32>) -> Self {
        Matrix2 {
            data: vec![
                mat.x.x as f64,
                mat.y.x as f64,
                mat.x.y as f64,
                mat.y.y as f64,
            ],
        }
    }

    pub fn to_matrix2(&self) -> ThreeDMatrix2<f32> {
        let b: Vec<f32> = self.data.iter().map(|&x| x as f32).collect();
        ThreeDMatrix2 {
            x: ThreeDVector2::new(b[0], b[1]),
            y: ThreeDVector2::new(b[2], b[3]),
        }
    }
}

/// Represents a 3x3 Matrix (stored column-major).
/// Stored internally as f32 for three-d, exposed as f64 for JS.
#[napi]
#[derive(Debug, Clone)]
pub struct Matrix3 {
    // Stored as a flat array of 9 f64s (column-major)
    pub data: Vec<f64>,
}

#[napi]
impl Matrix3 {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> Self {
        let d = data.unwrap_or_else(|| {
            // Return identity matrix by default
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        });
        if d.len() != 9 {
            panic!("Matrix3 data must have 9 elements");
        }
        Matrix3 { data: d }
    }

    #[napi]
    pub fn identity() -> Self {
        Matrix3 {
            data: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn from_matrix3(mat: &ThreeDMatrix3<f32>) -> Self {
        Matrix3 {
            data: vec![
                mat.x.x as f64,
                mat.y.x as f64,
                mat.z.x as f64,
                mat.x.y as f64,
                mat.y.y as f64,
                mat.z.y as f64,
                mat.x.z as f64,
                mat.y.z as f64,
                mat.z.z as f64,
            ],
        }
    }

    pub fn to_matrix3(&self) -> ThreeDMatrix3<f32> {
        let b: Vec<f32> = self.data.iter().map(|&x| x as f32).collect();
        ThreeDMatrix3 {
            x: ThreeDVector3::new(b[0], b[1], b[2]),
            y: ThreeDVector3::new(b[3], b[4], b[5]),
            z: ThreeDVector3::new(b[6], b[7], b[8]),
        }
    }
}

/// Represents a 4x4 Matrix (Transformation).
/// Stored internally as f32 for three-d, exposed as f64 for JS.
#[napi]
#[derive(Debug, Clone)]
pub struct Matrix4 {
    // Stored as a flat array of 16 f64s (column-major)
    pub data: Vec<f64>,
}

#[napi]
impl Matrix4 {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> Self {
        let d = data.unwrap_or_else(|| {
            // Return identity matrix by default
            vec![0.0; 16]
        });
        if d.len() != 16 {
            panic!("Matrix4 data must have 16 elements");
        }
        Matrix4 { data: d }
    }

    #[napi]
    pub fn identity() -> Self {
        let mat = ThreeDMatrix4::<f32>::identity();
        Matrix4::from_matrix4(&mat)
    }

    pub fn from_matrix4(mat: &ThreeDMatrix4<f32>) -> Self {
        let mut data = Vec::with_capacity(16);
        for i in 0..4 {
            let col = mat[i];
            data.push(col.x as f64);
            data.push(col.y as f64);
            data.push(col.z as f64);
            data.push(col.w as f64);
        }
        Matrix4 { data }
    }

    /// Returns the internal cgmath Matrix4 representation for use in three-d.
    pub fn to_matrix4(&self) -> ThreeDMatrix4<f32> {
        // Convert f64 back to f32
        let data_f32: Vec<f32> = self.data.iter().map(|&x| x as f32).collect();
        let b: &[f32] = &data_f32;
        // cgmath Matrix4 is stored as columns (x, y, z, w)
        // Input data is assumed to be column-major
        three_d::Matrix4 {
            x: ThreeDVector4::new(b[0], b[1], b[2], b[3]),
            y: ThreeDVector4::new(b[4], b[5], b[6], b[7]),
            z: ThreeDVector4::new(b[8], b[9], b[10], b[11]),
            w: ThreeDVector4::new(b[12], b[13], b[14], b[15]),
        }
    }
}

/// Represents a Rotation Angle in Degrees.
#[napi]
#[derive(Debug, Clone)]
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

impl From<&NDeg> for ThreeDDeg<f32> {
    fn from(d: &NDeg) -> Self {
        ThreeDDeg(d.value as f32)
    }
}

/// Represents a Rotation Angle in Radians.
#[napi]
#[derive(Debug, Clone)]
pub struct NRad {
    pub value: f64,
}

#[napi]
impl NRad {
    #[napi(constructor)]
    pub fn new(value: f64) -> Self {
        NRad { value }
    }
}

impl From<&NRad> for ThreeDRad<f32> {
    fn from(r: &NRad) -> Self {
        ThreeDRad(r.value as f32)
    }
}

/// Represents a Quaternion for rotation.
#[napi]
#[derive(Debug, Clone)]
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

impl From<&NQuaternion> for ThreeDQuaternion<f32> {
    fn from(q: &NQuaternion) -> Self {
        ThreeDQuaternion::new(q.w as f32, q.x as f32, q.y as f32, q.z as f32)
    }
}

/// Represents a 24-bit half-precision floating point number.
#[napi]
#[derive(Debug, Clone)]
pub struct NF16 {
    pub value: f64,
}

#[napi]
impl NF16 {
    #[napi(constructor)]
    pub fn new(value: f64) -> Self {
        NF16 { value }
    }
}

/// Represents an sRGB color with alpha channel.
#[napi]
#[derive(Debug, Clone)]
pub struct NSrgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[napi]
impl NSrgba {
    #[napi(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        NSrgba { r, g, b, a }
    }
}

impl From<&NSrgba> for ThreeDSrgba {
    fn from(q: &NSrgba) -> Self {
        ThreeDSrgba::new(
            (q.r * 255.0) as u8,
            (q.g * 255.0) as u8,
            (q.b * 255.0) as u8,
            (q.a * 255.0) as u8,
        )
    }
}

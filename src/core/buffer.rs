use napi_derive::napi;

/// Buffer usage hint for GPU buffers.
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

/// Represents a GPU Vertex Buffer Object (VBO) wrapper.
/// Vertex buffers store per-vertex data like positions, normals, colors, etc.
#[napi]
pub struct VertexBuffer {
    #[allow(dead_code)]
    buffer_type: String, // "Vertex"
    name: Option<String>,
    pub vertex_count: u32,
    /// Buffer usage hint
    usage: Option<BufferUsage>,
}

#[napi]
impl VertexBuffer {
    /// Creates a new vertex buffer wrapper.
    #[napi(constructor)]
    pub fn new(name: Option<String>, vertex_count: u32, usage: Option<String>) -> Self {
        let buffer_usage = usage.as_ref().map(|u| match u.as_str() {
            "static_draw" => BufferUsage::StaticDraw,
            "dynamic_draw" => BufferUsage::DynamicDraw,
            "stream_draw" => BufferUsage::StreamDraw,
            _ => BufferUsage::StaticDraw,
        });

        VertexBuffer {
            buffer_type: "Vertex".to_string(),
            name,
            vertex_count,
            usage: buffer_usage,
        }
    }

    /// Returns usage string for JavaScript.
    #[napi]
    pub fn get_usage(&self) -> String {
        match self.usage {
            Some(BufferUsage::StaticDraw) => "static_draw".to_string(),
            Some(BufferUsage::DynamicDraw) => "dynamic_draw".to_string(),
            Some(BufferUsage::StreamDraw) => "stream_draw".to_string(),
            Some(BufferUsage::StaticRead) => "static_read".to_string(),
            Some(BufferUsage::DynamicRead) => "dynamic_read".to_string(),
            Some(BufferUsage::StreamRead) => "stream_read".to_string(),
            Some(BufferUsage::StaticCopy) => "static_copy".to_string(),
            Some(BufferUsage::DynamicCopy) => "dynamic_copy".to_string(),
            Some(BufferUsage::StreamCopy) => "stream_copy".to_string(),
            None => "static_draw".to_string(),
        }
    }

    /// Returns buffer info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "VertexBuffer(name={:?}, count={}, usage={})",
            self.name,
            self.vertex_count,
            self.get_usage()
        )
    }

    /// Checks if buffer is valid.
    #[napi]
    pub fn is_valid(&self) -> bool {
        self.vertex_count > 0
    }

    /// Returns buffer size in bytes.
    /// Returns the estimated memory usage in bytes.
    #[napi]
    pub fn estimate_size_bytes(&self) -> u64 {
        // Assume typical float3 data (12 bytes per vertex)
        self.vertex_count as u64 * 12
    }

    /// Returns the buffer type as a string.
    #[napi]
    pub fn get_buffer_type(&self) -> String {
        self.buffer_type.clone()
    }
}

/// Represents an Element (Index) Buffer Object (EBO) wrapper.
/// Element buffers store vertex indices for indexed rendering.
#[napi]
pub struct ElementBuffer {
    #[allow(dead_code)]
    buffer_type: String, // "Element"
    name: Option<String>,
    pub index_count: u32,
    /// Buffer usage hint
    usage: Option<BufferUsage>,
}

#[napi]
impl ElementBuffer {
    /// Creates a new element buffer wrapper.
    #[napi(constructor)]
    pub fn new(name: Option<String>, index_count: u32, usage: Option<String>) -> Self {
        let buffer_usage = usage.as_ref().map(|u| match u.as_str() {
            "static_draw" => BufferUsage::StaticDraw,
            "dynamic_draw" => BufferUsage::DynamicDraw,
            "stream_draw" => BufferUsage::StreamDraw,
            _ => BufferUsage::StaticDraw,
        });

        ElementBuffer {
            buffer_type: "Element".to_string(),
            name,
            index_count,
            usage: buffer_usage,
        }
    }

    /// Returns usage string for JavaScript.
    #[napi]
    pub fn get_usage(&self) -> String {
        match self.usage {
            Some(BufferUsage::StaticDraw) => "static_draw".to_string(),
            Some(BufferUsage::DynamicDraw) => "dynamic_draw".to_string(),
            Some(BufferUsage::StreamDraw) => "stream_draw".to_string(),
            Some(BufferUsage::StaticRead) => "static_read".to_string(),
            Some(BufferUsage::DynamicRead) => "dynamic_read".to_string(),
            Some(BufferUsage::StreamRead) => "stream_read".to_string(),
            Some(BufferUsage::StaticCopy) => "static_copy".to_string(),
            Some(BufferUsage::DynamicCopy) => "dynamic_copy".to_string(),
            Some(BufferUsage::StreamCopy) => "stream_copy".to_string(),
            None => "static_draw".to_string(),
        }
    }

    /// Returns buffer info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "ElementBuffer(name={:?}, count={}, usage={})",
            self.name,
            self.index_count,
            self.get_usage()
        )
    }

    /// Checks if buffer is valid.
    #[napi]
    pub fn is_valid(&self) -> bool {
        self.index_count > 0
    }

    /// Returns the estimated memory usage in bytes.
    #[napi]
    pub fn estimate_size_bytes(&self) -> u64 {
        // Assume UINT32 indices (4 bytes per index)
        self.index_count as u64 * 4
    }

    /// Returns the buffer type as a string.
    #[napi]
    pub fn get_buffer_type(&self) -> String {
        self.buffer_type.clone()
    }
}

/// Represents a generic Buffer to demonstrate type safety.
/// This is a base type hint for buffer classification.
#[napi]
#[derive(Debug, Clone)]
pub enum BufferType {
    Vertex,
    Element,
    Instance,
    Uniform,
}

/// Represents an Instance Buffer Object (UBO) wrapper.
/// Instance buffers store per-instance transformation/matrix data (for instanced rendering).
#[napi]
pub struct InstanceBuffer {
    #[allow(dead_code)]
    buffer_type: String, // "Instance"
    name: Option<String>,
    pub instance_count: u32,
    /// Float data can be provided for validation (using f64 for N-API compatibility)
    pub instance_data: Option<Vec<f64>>,
}

#[napi]
impl InstanceBuffer {
    /// Creates a new instance buffer wrapper.
    #[napi(constructor)]
    pub fn new(name: Option<String>, instance_count: u32, instance_data: Option<Vec<f64>>) -> Self {
        InstanceBuffer {
            buffer_type: "Instance".to_string(),
            name,
            instance_count,
            instance_data,
        }
    }

    /// Returns buffer info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "InstanceBuffer(name={:?}, count={})",
            self.name, self.instance_count
        )
    }

    /// Checks if buffer is valid.
    #[napi]
    pub fn is_valid(&self) -> bool {
        self.instance_count > 0
    }

    /// Returns estimated size in bytes.
    #[napi]
    pub fn estimate_size_bytes(&self) -> u64 {
        // Worst case, typical instance data has 16 floats per instance (4x4 matrix)
        match &self.instance_data {
            Some(data) => (data.len() * 8) as u64,    // 8 bytes per f64
            None => self.instance_count as u64 * 128, // 16 floats * 8 bytes
        }
    }

    /// Returns the buffer type as a string.
    #[napi]
    pub fn get_buffer_type(&self) -> String {
        self.buffer_type.clone()
    }
}

/// Represents a Uniform Buffer Object (UBO) wrapper.
/// UBOs store uniforms shared across multiple shader programs (efficient for batch updates).
#[napi]
pub struct UniformBuffer {
    buffer_type: String, // "Uniform"
    name: Option<String>,
    pub binding_point: u32,
    pub size_bytes: u32,
}

#[napi]
impl UniformBuffer {
    /// Creates a new uniform buffer wrapper.
    #[napi(constructor)]
    pub fn new(name: Option<String>, binding_point: u32, size_bytes: Option<u32>) -> Self {
        UniformBuffer {
            buffer_type: "Uniform".to_string(),
            name,
            binding_point,
            size_bytes: size_bytes.unwrap_or(0),
        }
    }

    /// Returns buffer info string.
    #[napi]
    pub fn get_info(&self) -> String {
        format!(
            "UniformBuffer(name={:?}, binding={}, size={} bytes)",
            self.name, self.binding_point, self.size_bytes
        )
    }

    /// Returns the buffer type as a string.
    #[napi]
    pub fn get_buffer_type(&self) -> String {
        self.buffer_type.clone()
    }

    /// Checks if buffer is valid.
    #[napi]
    pub fn is_valid(&self) -> bool {
        self.size_bytes > 0
    }

    /// Returns buffer size in bytes.
    #[napi]
    pub fn get_size_bytes(&self) -> u32 {
        self.size_bytes
    }
}

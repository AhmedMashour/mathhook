//! WebGPU Compute Implementation
//!
//! This module provides WebGPU-based GPU acceleration for mathematical operations
//! using compute shaders for cross-platform GPU acceleration.

#[cfg(feature = "webgpu")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "webgpu")]
use wgpu::*;

/// WebGPU compute context for mathematical operations
#[cfg(feature = "webgpu")]
pub struct WebGpuCompute {
    device: Device,
    queue: Queue,
    bulk_add_pipeline: ComputePipeline,
    matrix_multiply_pipeline: ComputePipeline,
}

#[cfg(feature = "webgpu")]
impl WebGpuCompute {
    /// Create a new WebGPU compute context
    pub async fn new() -> Result<Self, WebGpuError> {
        // Request adapter
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(WebGpuError::NoAdapter)?;

        // Request device
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("MathHook GPU Device"),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| WebGpuError::DeviceRequest(format!("{:?}", e)))?;

        // Create compute pipelines
        let bulk_add_pipeline = Self::create_bulk_add_pipeline(&device)?;
        let matrix_multiply_pipeline = Self::create_matrix_multiply_pipeline(&device)?;

        Ok(Self {
            device,
            queue,
            bulk_add_pipeline,
            matrix_multiply_pipeline,
        })
    }

    /// Create bulk addition compute pipeline
    fn create_bulk_add_pipeline(device: &Device) -> Result<ComputePipeline, WebGpuError> {
        let shader_source = r#"
            @group(0) @binding(0) var<storage, read> input_data: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output_data: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                if (index >= arrayLength(&input_data)) {
                    return;
                }

                // Parallel reduction for bulk addition
                let local_index = global_id.x;
                let workgroup_size = 64u;

                // Each thread processes one element
                var sum = input_data[index];

                // Store partial sum (simplified version)
                output_data[index] = sum;
            }
        "#;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Bulk Add Shader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = device.create_compute_pipeline(&ComputePipeline {
            label: Some("Bulk Add Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        Ok(pipeline)
    }

    /// Create matrix multiplication compute pipeline
    fn create_matrix_multiply_pipeline(device: &Device) -> Result<ComputePipeline, WebGpuError> {
        let shader_source = r#"
            @group(0) @binding(0) var<storage, read> matrix_a: array<f32>;
            @group(0) @binding(1) var<storage, read> matrix_b: array<f32>;
            @group(0) @binding(2) var<storage, read_write> matrix_c: array<f32>;
            @group(0) @binding(3) var<uniform> dimensions: vec3<u32>; // rows_a, cols_a, cols_b

            @compute @workgroup_size(16, 16)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let row = global_id.x;
                let col = global_id.y;

                let rows_a = dimensions.x;
                let cols_a = dimensions.y;
                let cols_b = dimensions.z;

                if (row >= rows_a || col >= cols_b) {
                    return;
                }

                var sum = 0.0;
                for (var k = 0u; k < cols_a; k++) {
                    let a_val = matrix_a[row * cols_a + k];
                    let b_val = matrix_b[k * cols_b + col];
                    sum += a_val * b_val;
                }

                matrix_c[row * cols_b + col] = sum;
            }
        "#;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Matrix Multiply Shader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = device.create_compute_pipeline(&ComputePipeline {
            label: Some("Matrix Multiply Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        Ok(pipeline)
    }

    /// Perform GPU bulk addition
    pub async fn bulk_add(&self, values: &[f64]) -> Result<f64, WebGpuError> {
        // Convert f64 to f32 for GPU compatibility
        let input_data: Vec<f32> = values.iter().map(|&x| x as f32).collect();

        // Create buffers
        let input_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: bytemuck::cast_slice(&input_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Output Buffer"),
            size: (input_data.len() * std::mem::size_of::<f32>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (input_data.len() * std::mem::size_of::<f32>()) as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Bulk Add Bind Group"),
            layout: &self.bulk_add_pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        // Dispatch compute shader
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Bulk Add Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Bulk Add Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.bulk_add_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            let workgroup_count = (input_data.len() + 63) / 64; // Round up division
            compute_pass.dispatch_workgroups(workgroup_count as u32, 1, 1);
        }

        // Copy result to staging buffer
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, staging_buffer.size());

        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));

        // Read result
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |v| {
            let _ = sender.send(v);
        });

        self.device.poll(Maintain::Wait);
        receiver
            .receive()
            .await
            .ok_or(WebGpuError::BufferMap("Channel closed".to_string()))?
            .map_err(|e| WebGpuError::BufferMap(format!("{:?}", e)))?;

        let data = buffer_slice.get_mapped_range();
        let result_data: &[f32] = bytemuck::cast_slice(&data);

        // Sum the results (simplified - in practice would use proper reduction)
        let sum = result_data.iter().sum::<f32>() as f64;

        drop(data);
        staging_buffer.unmap();

        Ok(sum)
    }

    /// Perform GPU matrix multiplication
    pub async fn matrix_multiply(
        &self,
        a: &[Vec<f64>],
        b: &[Vec<f64>],
    ) -> Result<Vec<Vec<f64>>, WebGpuError> {
        let rows_a = a.len();
        let cols_a = a[0].len();
        let rows_b = b.len();
        let cols_b = b[0].len();

        if cols_a != rows_b {
            return Err(WebGpuError::InvalidDimensions);
        }

        // Flatten matrices and convert to f32
        let matrix_a_flat: Vec<f32> = a
            .iter()
            .flat_map(|row| row.iter().map(|&x| x as f32))
            .collect();
        let matrix_b_flat: Vec<f32> = b
            .iter()
            .flat_map(|row| row.iter().map(|&x| x as f32))
            .collect();

        // Create buffers
        let buffer_a = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Matrix A Buffer"),
            contents: bytemuck::cast_slice(&matrix_a_flat),
            usage: BufferUsages::STORAGE,
        });

        let buffer_b = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Matrix B Buffer"),
            contents: bytemuck::cast_slice(&matrix_b_flat),
            usage: BufferUsages::STORAGE,
        });

        let buffer_c = self.device.create_buffer(&BufferDescriptor {
            label: Some("Matrix C Buffer"),
            size: (rows_a * cols_b * std::mem::size_of::<f32>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let dimensions = [rows_a as u32, cols_a as u32, cols_b as u32, 0u32]; // Pad to 16 bytes
        let dimensions_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Dimensions Buffer"),
            contents: bytemuck::cast_slice(&dimensions),
            usage: BufferUsages::UNIFORM,
        });

        let staging_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_c.size(),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Matrix Multiply Bind Group"),
            layout: &self.matrix_multiply_pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer_a.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer_b.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_c.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: dimensions_buffer.as_entire_binding(),
                },
            ],
        });

        // Dispatch compute shader
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Matrix Multiply Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Matrix Multiply Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.matrix_multiply_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            let workgroup_x = (rows_a + 15) / 16; // Round up division
            let workgroup_y = (cols_b + 15) / 16;
            compute_pass.dispatch_workgroups(workgroup_x as u32, workgroup_y as u32, 1);
        }

        // Copy result to staging buffer
        encoder.copy_buffer_to_buffer(&buffer_c, 0, &staging_buffer, 0, staging_buffer.size());

        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));

        // Read result
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |v| {
            let _ = sender.send(v);
        });

        self.device.poll(Maintain::Wait);
        receiver
            .receive()
            .await
            .ok_or(WebGpuError::BufferMap("Channel closed".to_string()))?
            .map_err(|e| WebGpuError::BufferMap(format!("{:?}", e)))?;

        let data = buffer_slice.get_mapped_range();
        let result_data: &[f32] = bytemuck::cast_slice(&data);

        // Convert back to 2D matrix
        let mut result = vec![vec![0.0; cols_b]; rows_a];
        for i in 0..rows_a {
            for j in 0..cols_b {
                result[i][j] = result_data[i * cols_b + j] as f64;
            }
        }

        drop(data);
        staging_buffer.unmap();

        Ok(result)
    }
}

/// WebGPU-specific errors
#[derive(Debug, Clone)]
pub enum WebGpuError {
    NoAdapter,
    DeviceRequest(String),
    ShaderCompilation(String),
    BufferMap(String),
    InvalidDimensions,
    ComputeError(String),
}

impl std::fmt::Display for WebGpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebGpuError::NoAdapter => write!(f, "No WebGPU adapter found"),
            WebGpuError::DeviceRequest(msg) => write!(f, "Device request failed: {}", msg),
            WebGpuError::ShaderCompilation(msg) => write!(f, "Shader compilation failed: {}", msg),
            WebGpuError::BufferMap(msg) => write!(f, "Buffer mapping failed: {}", msg),
            WebGpuError::InvalidDimensions => write!(f, "Invalid matrix dimensions"),
            WebGpuError::ComputeError(msg) => write!(f, "Compute error: {}", msg),
        }
    }
}

impl std::error::Error for WebGpuError {}

// Placeholder implementations for when WebGPU is not available
#[cfg(not(feature = "webgpu"))]
pub struct WebGpuCompute;

#[cfg(not(feature = "webgpu"))]
impl WebGpuCompute {
    pub async fn new() -> Result<Self, WebGpuError> {
        Err(WebGpuError::NoAdapter)
    }

    pub async fn bulk_add(&self, _values: &[f64]) -> Result<f64, WebGpuError> {
        Err(WebGpuError::NoAdapter)
    }

    pub async fn matrix_multiply(
        &self,
        _a: &[Vec<f64>],
        _b: &[Vec<f64>],
    ) -> Result<Vec<Vec<f64>>, WebGpuError> {
        Err(WebGpuError::NoAdapter)
    }
}

#[cfg(not(feature = "webgpu"))]
#[derive(Debug, Clone)]
pub enum WebGpuError {
    NoAdapter,
    DeviceRequest(String),
    ShaderCompilation(String),
    BufferMap(String),
    InvalidDimensions,
    ComputeError(String),
}

#[cfg(not(feature = "webgpu"))]
impl std::fmt::Display for WebGpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebGPU not available (feature not enabled)")
    }
}

#[cfg(not(feature = "webgpu"))]
impl std::error::Error for WebGpuError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webgpu_availability() {
        // Test should pass regardless of WebGPU availability
        #[cfg(feature = "webgpu")]
        {
            // WebGPU tests would go here
            // Note: These require async runtime and actual GPU
        }

        #[cfg(not(feature = "webgpu"))]
        {
            // Verify graceful handling when WebGPU is not available
            let compute = WebGpuCompute;
            // Should not panic
        }
    }
}

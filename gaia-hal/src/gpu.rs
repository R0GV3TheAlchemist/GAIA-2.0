//! GPU / NPU subsystem — compute accelerator surface trait.

/// Compute backend (graphics API or ML runtime).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuBackend {
    /// NVIDIA CUDA.
    Cuda,
    /// AMD ROCm / HIP.
    RoCm,
    /// Khronos OpenCL.
    OpenCl,
    /// Apple Metal.
    Metal,
    /// Vendor-neutral Vulkan compute.
    Vulkan,
    /// Neural Processing Unit (vendor-specific).
    Npu,
    /// Software / CPU fallback.
    None,
}

/// Metadata about a GPU or NPU device.
#[derive(Debug, Clone)]
pub struct GpuDevice {
    /// Device index.
    pub index: u32,
    /// Human-readable device name.
    pub name: String,
    /// Available compute backend.
    pub backend: GpuBackend,
    /// Total device memory in bytes (0 if unknown).
    pub memory_bytes: u64,
}

impl GpuDevice {
    /// Returns `true` if this device can be used for ML inference.
    pub fn ml_capable(&self) -> bool {
        !matches!(self.backend, GpuBackend::None)
    }
}

/// Enumerate GPU/NPU devices visible to the current process.
///
/// At Tier 0/1 this returns an empty list unless a backend is
/// explicitly configured. Real device discovery (CUDA, ROCm) will
/// be wired in future crate versions.
pub fn enumerate_devices() -> Vec<GpuDevice> {
    // Phase 1: userspace stub. Returns empty unless a mock is injected.
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_does_not_panic() {
        let devs = enumerate_devices();
        // May be empty at T0 — that is fine.
        for d in &devs {
            assert!(!d.name.is_empty());
        }
    }

    #[test]
    fn none_backend_is_not_ml_capable() {
        let d = GpuDevice { index: 0, name: "cpu-fallback".into(), backend: GpuBackend::None, memory_bytes: 0 };
        assert!(!d.ml_capable());
    }

    #[test]
    fn cuda_backend_is_ml_capable() {
        let d = GpuDevice { index: 0, name: "RTX 4090".into(), backend: GpuBackend::Cuda, memory_bytes: 24 * 1024 * 1024 * 1024 };
        assert!(d.ml_capable());
    }
}

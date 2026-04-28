use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComputeType {
    Inference,
    PreTraining,
    RLTraining,
    Scaling,
    DataProcessing,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComputeArena {
    /// GPU inference (FLOPS optimized)
    GPUInference {
        gpu_type: String, // A100, H100, etc.
        vram_gb: u16,
    },
    /// Tensor parallelism
    DistributedTraining {
        num_nodes: u32,
        gpu_type: String,
    },
    /// Reinforcement Learning specific
    RLEnvironment {
        parallel_envs: u32,
        framework: String, // PyTorch, TensorFlow, etc.
    },
    /// General compute with scaling laws
    ScaledCompute {
        tflops: u64,
        memory_gb: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResource {
    pub id: String,
    pub compute_type: ComputeType,
    pub arena: ComputeArena,
    pub price_per_hour: Decimal,
    pub available_units: u32,
    pub min_duration_hours: u32,
    pub max_duration_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeSpecification {
    pub architecture: String,
    pub memory_gb: u32,
    pub storage_gb: u64,
    pub network_bandwidth_gbps: f32,
    pub supports_distributed: bool,
    pub quantum_safe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeOrder {
    pub resource_id: String,
    pub quantity: u32,
    pub duration_hours: u32,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub specifications: ComputeSpecification,
}

impl ComputeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Inference => "inference",
            Self::PreTraining => "pre-training",
            Self::RLTraining => "rl-training",
            Self::Scaling => "scaling",
            Self::DataProcessing => "data-processing",
            Self::Custom(s) => s.as_str(),
        }
    }
}

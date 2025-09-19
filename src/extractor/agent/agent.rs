use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::extractor::{Extractor, agent::AgentError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentOs {
    pub distribution: String,
    pub distribution_like: String,
    pub name: String,
    pub version: String,
    pub long_version: String,
    pub kernel: Option<String>,
    pub architecture: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ByteAmount {
    pub bytes: u64,
    pub human: String,
}
impl ByteAmount {
    pub fn new(bytes: u64) -> Self {
        let human = humansize::format_size(bytes, humansize::DECIMAL);
        ByteAmount { bytes, human }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentMemory {
    pub total: ByteAmount,
    pub used: ByteAmount,
    pub free: ByteAmount,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    pub hostname: String,
    pub os: AgentOs,
    pub memory: AgentMemory,
    pub swap: AgentMemory,
    pub ncpus: usize,
}

impl Extractor for Agent {
    type Error = AgentError;

    fn extract() -> Result<Self, Self::Error> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or("Unknown Host".to_string());
        let os = AgentOs {
            distribution: System::distribution_id(),
            distribution_like: System::distribution_id_like().join(","),
            name: System::name().unwrap_or("Unknown OS".to_string()),
            version: System::kernel_version().unwrap_or("Unknown Version".to_string()),
            long_version: System::long_os_version().unwrap_or("Unknown Long Version".to_string()),
            kernel: System::os_version(),
            architecture: System::cpu_arch(),
        };
        let memory = AgentMemory {
            total: ByteAmount::new(sys.total_memory()),
            used: ByteAmount::new(sys.used_memory()),
            free: ByteAmount::new(sys.free_memory()),
        };
        let swap = AgentMemory {
            total: ByteAmount::new(sys.total_swap()),
            used: ByteAmount::new(sys.used_swap()),
            free: ByteAmount::new(sys.free_swap()),
        };
        let ncpus = sys.cpus().len();

        Ok(Agent {
            hostname,
            os,
            memory,
            swap,
            ncpus,
        })
    }
}

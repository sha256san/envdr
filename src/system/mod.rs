pub mod package_manager;

use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReport {
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub arch: String,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub cpu_count: usize,
    pub cpu_brand: String,
}

impl SystemReport {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os = System::name().unwrap_or_else(|| std::env::consts::OS.to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let host_name = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let arch = std::env::consts::ARCH.to_string();

        let total_memory_mb = sys.total_memory() / (1024 * 1024);
        let used_memory_mb = sys.used_memory() / (1024 * 1024);
        let cpu_count = sys.cpus().len();
        let cpu_brand = sys.cpus().first()
            .map(|cpu| cpu.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        Self {
            os,
            os_version,
            kernel_version,
            host_name,
            arch,
            total_memory_mb,
            used_memory_mb,
            cpu_count,
            cpu_brand,
        }
    }
}

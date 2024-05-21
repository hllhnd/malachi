use std::fs::read_to_string;

use super::Nugget;
use super::Widget;

#[cfg(target_arch = "x86_64")]
mod x86;

/// A widget that displays information about the CPU.
pub struct CpuModel;

impl CpuModel {
	fn maximum_frequency(&self) -> u64 {
		read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
			.unwrap()
			.trim_end()
			.parse::<u64>()
			.unwrap()
	}

	fn model(&self) -> String {
		#[cfg(target_arch = "x86_64")]
		{
			String::from_utf8_lossy(&x86::cpu_model())
				.trim_end_matches([' ', '\0'])
				.to_string()
		}
	}
}

impl Widget for CpuModel {
	fn nuggets(&self) -> Vec<Nugget> {
		vec![Nugget::new(
			"CPU",
			format!(
				"{} @ {:.2} GHz",
				self.model(),
				self.maximum_frequency() as f64 / 1_000_000.0
			),
		)]
	}
}

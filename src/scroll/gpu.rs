use std::mem::transmute;

use ash::vk;
use ash::vk::InstanceCreateInfo;
use ash::Entry;
use color_eyre::Result;

use super::Scroll;
use super::Spell;

/// A scroll that displays information about the GPU. So far it displays the model and the max supported Vulkan API
/// version.
pub struct GpuInfo;

impl Scroll for GpuInfo {
	fn spells(&self) -> Result<Vec<Spell>> {
		let entry = unsafe { Entry::load() }?;
		let create_info = InstanceCreateInfo::default();
		let instance = unsafe { entry.create_instance(&create_info, None) }?;
		let physical_devices = unsafe { instance.enumerate_physical_devices() }?;

		let mut spells = Vec::new();
		for device in physical_devices {
			let properties = unsafe { instance.get_physical_device_properties(device) };
			let device_name: [u8; 256] = unsafe { transmute(properties.device_name) };
			let model = String::from_utf8_lossy(&device_name).trim_end().to_string();
			let api_ver = format!(
				"{}.{}.{}",
				vk::api_version_major(properties.api_version),
				vk::api_version_minor(properties.api_version),
				vk::api_version_patch(properties.api_version)
			);
			spells.push(Spell::new("GPU", format!("{model} (Vulkan {api_ver})")));
		}

		Ok(spells)
	}
}

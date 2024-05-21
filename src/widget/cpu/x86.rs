use std::arch::x86_64::__cpuid;

/// Helper for getting the CPU model on x86-based systems. The returned result is probably not useful on its own, and
/// should be converted to a string. Note that neither Intel nor AMD are consistent at all about whether this is null
/// terminated. Just assume it isn't and trim anything that is a space or null byte.
pub fn cpu_model() -> [u8; 48] {
	let mut buffer = [0; 48];

	for i in 0..=2 {
		let leaf = i + 0x80000002;
		let cpuid = unsafe { __cpuid(leaf) };

		let offset = (i * 16) as usize;
		buffer[offset..offset + 4].copy_from_slice(&cpuid.eax.to_le_bytes());
		buffer[offset + 4..offset + 8].copy_from_slice(&cpuid.ebx.to_le_bytes());
		buffer[offset + 8..offset + 12].copy_from_slice(&cpuid.ecx.to_le_bytes());
		buffer[offset + 12..offset + 16].copy_from_slice(&cpuid.edx.to_le_bytes());
	}

	buffer
}

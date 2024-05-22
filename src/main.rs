mod scroll;

use color_eyre::Result;
use scroll::cpu::CpuModel;
use scroll::gpu::GpuInfo;
use scroll::Sorcerer;

fn main() -> Result<()> {
	color_eyre::install()?;

	let mut sorcerer = Sorcerer::new();
	sorcerer.add_scroll(Box::new(CpuModel));
	sorcerer.add_scroll(Box::new(GpuInfo));
	sorcerer.print()?;

	Ok(())
}

mod widget;

use color_eyre::Result;
use widget::cpu::CpuModel;
use widget::gpu::GpuInfo;
use widget::WidgetPrinter;

fn main() -> Result<()> {
	color_eyre::install()?;

	let mut wp = WidgetPrinter::new();
	wp.add_widget(Box::new(CpuModel));
	wp.add_widget(Box::new(GpuInfo));
	wp.print()?;

	Ok(())
}

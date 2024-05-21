mod widget;

use widget::WidgetPrinter;

fn main() {
	let mut wp = WidgetPrinter::new();
	wp.add_widget(Box::new(widget::cpu::CpuModel));
	wp.add_widget(Box::new(widget::gpu::GpuInfo));
	wp.print();
}

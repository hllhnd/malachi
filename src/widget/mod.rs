use color_print::cprintln;

pub mod cpu;
pub mod gpu;

pub trait Widget {
	fn nuggets(&self) -> Vec<Nugget>;
}

#[derive(Debug)]
pub struct Nugget {
	pub name:  &'static str,
	pub value: String,
}

impl Nugget {
	pub fn new(name: &'static str, value: String) -> Self {
		Self { name, value }
	}
}

pub struct WidgetPrinter {
	widgets: Vec<Box<dyn Widget>>,
}

impl WidgetPrinter {
	pub fn new() -> Self {
		Self { widgets: Vec::new() }
	}

	pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
		self.widgets.push(widget);
	}

	pub fn print(&self) {
		let nuggets: Vec<Nugget> = self.widgets.iter().flat_map(|widget| widget.nuggets()).collect();
		let longest_name = nuggets.iter().map(|nugget| nugget.name.len()).max().unwrap_or(0);
		for nugget in nuggets {
			cprintln!(
				"<bold><bright-blue>{}:</bright-blue></bold>{}{}",
				nugget.name,
				" ".repeat(longest_name - nugget.name.len() + 1),
				nugget.value
			);
		}
	}
}

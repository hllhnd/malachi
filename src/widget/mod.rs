pub mod cpu;
pub mod gpu;

use color_eyre::Result;
use color_print::cprintln;

/// A widget is a component that provides information about a specific aspect of the system.
///
/// A widget encompasses broad but individual categories like CPU, GPU, and memory, and may provide multiple [Nugget]s
/// of information.
pub trait Widget {
	/// Get all nuggets of information provided by this widget.
	///
	/// This is usually an expensive call, as widgets do not necessarily need to cache their information.
	fn nuggets(&self) -> Result<Vec<Nugget>>;
}

/// A name-value pairing for a subunit of a [Widget]. Named this way because it's a nugget of information :)
#[derive(Debug)]
pub struct Nugget {
	/// The name of the nugget. This is what will be printed to the left of the value.
	pub name:  &'static str,
	/// The string representation of the value of the nugget.
	pub value: String,
}

impl Nugget {
	/// Create a new Nugget with a name and value.
	pub fn new(name: &'static str, value: String) -> Self {
		Self { name, value }
	}
}

/// A printer for widgets. Comes with pretty colors (blue).
pub struct WidgetPrinter {
	widgets: Vec<Box<dyn Widget>>,
}

impl WidgetPrinter {
	/// Create a new WidgetPrinter with no widgets.
	pub fn new() -> Self {
		Self { widgets: Vec::new() }
	}

	/// Register a widget to be printed.
	pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
		self.widgets.push(widget);
	}

	/// Print all registered widgets to stdout.
	pub fn print(&self) -> Result<()> {
		let nuggets = {
			let mut nuggets = Vec::new();
			for widget in &self.widgets {
				nuggets.extend(widget.nuggets()?);
			}
			nuggets
		};

		let longest_name = nuggets.iter().map(|nugget| nugget.name.len()).max().unwrap_or(0);
		for nugget in nuggets {
			cprintln!(
				"<bold><bright-blue>{}:</bright-blue></bold>{}{}",
				nugget.name,
				" ".repeat(longest_name - nugget.name.len() + 1),
				nugget.value
			);
		}

		Ok(())
	}
}

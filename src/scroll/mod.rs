pub mod cpu;
pub mod gpu;

use color_eyre::Result;
use color_print::cprintln;

/// A spell is a component that provides information about a specific aspect of the system.
///
/// A spell encompasses broad but individual categories like CPU, GPU, and memory, and may provide multiple [Spell]s.
pub trait Scroll {
	/// Retrieve the spell(s) which this scroll can cast.
	///
	/// This is usually an expensive call, as scrolls do not need to cache their information.
	fn spells(&self) -> Result<Vec<Spell>>;
}

/// A name-value pairing for the output(s) of a [Scroll].
#[derive(Debug)]
pub struct Spell {
	/// The name of the spell. This is what will be printed to the left of the value.
	pub name:  &'static str,
	/// The string representation of the value of the spell.
	pub value: String,
}

impl Spell {
	/// Create a new spell with a name and value.
	pub fn new(name: &'static str, value: String) -> Self {
		Self { name, value }
	}
}

/// A printer for scrolls. Creates pretty-printed output and prints it to stdout.
pub struct Sorcerer {
	scrolls: Vec<Box<dyn Scroll>>,
}

impl Sorcerer {
	/// Create a new sorcerer with no scrolls registered
	pub fn new() -> Self {
		Self { scrolls: Vec::new() }
	}

	/// Register a scroll to be printed.
	pub fn add_scroll(&mut self, scroll: Box<dyn Scroll>) {
		self.scrolls.push(scroll);
	}

	/// Print all registered scrolls to stdout.
	pub fn print(&self) -> Result<()> {
		let spells = {
			let mut scrolls = Vec::new();
			for scroll in &self.scrolls {
				scrolls.extend(scroll.spells()?);
			}
			scrolls
		};

		let longest_name = spells.iter().map(|spell| spell.name.len()).max().unwrap_or(0);
		for spell in spells {
			cprintln!(
				"<bold><bright-blue>{}:</bright-blue></bold>{}{}",
				spell.name,
				" ".repeat(longest_name - spell.name.len() + 1),
				spell.value
			);
		}

		Ok(())
	}
}

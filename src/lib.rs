pub mod api;
pub mod cli;

pub trait Executor {
	fn run(&mut self) -> Result<(), anyhow::Error>;
}
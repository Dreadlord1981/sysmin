use std::{env, fs::{self, OpenOptions}, io::Write, path::PathBuf, process::{Command, Stdio}, vec};
use css_minify::optimizations::{Level, Minifier};

use crate::Executor;

#[derive(Debug, Default, Clone)]
pub struct Operation<'a> {
	pub name: &'a str,
	pub out_path: PathBuf,
	pub debug_path: PathBuf,
	pub files: Vec<&'a str>,
	pub extra: Vec<&'a str>,
	pub append: bool
}

impl<'a> Operation<'a> {
	pub fn new(name: &'a str, out_path: &str, debug_path: &str) -> Self {

		let path = shellexpand::full(out_path).unwrap_or_else(|_| panic!("Could not expand path for file: {}", debug_path));
		let debug = shellexpand::full(debug_path).unwrap_or_else(|_| panic!("Could not expand path for file: {}", debug_path));

		let path_out = PathBuf::from(path.to_string());
		let debug_out = PathBuf::from(debug.to_string());

		Self {
			name,
			out_path: path_out,
			debug_path: debug_out,
			..Default::default()
		}
	}
}

impl<'a> Executor for Operation<'a> {
	fn run(&mut self) -> Result<(), anyhow::Error> {

		println!("{}: start", self.name);

		let temp_path = env::temp_dir();

		let mut out_system_debug_path = temp_path.clone();
		let mut out_system_path = temp_path;

		out_system_debug_path.push("out-debug.js");
		out_system_path.push("out.js");

		if out_system_debug_path.exists() {
			fs::remove_file(&out_system_debug_path)?;
		}
		
		let mut system_debug_file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(&out_system_debug_path)?;

		println!("{}: Concatenating files", self.name);

		for file_path in &self.files {

			let read_path = shellexpand::full(file_path)?;

			let result = fs::read(read_path.to_string())?;

			system_debug_file.write_all(&result)?;
		}

		println!("{}: Concatenating done", self.name);

		if out_system_path.exists() {
			fs::remove_file(&out_system_path)?;
		}

		println!("{}: Processing files", self.name);

		let mut child = Command::new("cmd")
		.args([
			"/C",
			"uglifyjs",
			&out_system_debug_path.to_string_lossy(),
			"-c",
			"toplevel",
			"-m",
			"toplevel",
			"-o",
			&out_system_path.to_string_lossy()
			
		])
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()?;

		child.wait()?;

		if self.append {

			println!("{}: Appeding extra", self.name);

			let mut system_file = OpenOptions::new()
			.create(true)
			.append(true)
			.open(&out_system_path)?;
				
			for file_path in &self.extra {

				let read_path = shellexpand::full(file_path)?;

				let content = fs::read(read_path.to_string())?;

				system_debug_file.write_all(&content)?;
				system_file.write_all(&content)?;
			}

			println!("{}: Appeding done", self.name);
		}

		fs::rename(&out_system_debug_path, &self.debug_path)?;
		fs::rename(&out_system_path, &self.out_path)?;

		println!("{}: done", self.name);
		println!();

		Ok(())
	}
}

#[derive(Debug, Default, Clone)]
pub struct CssOperation<'a> {
	pub name: &'a str,
	pub out_path: PathBuf,
	pub debug_path: PathBuf,
	pub files: Vec<&'a str>,
	pub extra: Vec<&'a str>,
	pub append: bool
}

impl<'a> CssOperation<'a> {
	pub fn new(name: &'a str, out_path: &str, debug_path: &str) -> Self {

		let path = shellexpand::full(out_path).unwrap_or_else(|_| panic!("Could not expand path for file: {}", debug_path));
		let debug = shellexpand::full(debug_path).unwrap_or_else(|_| panic!("Could not expand path for file: {}", debug_path));

		let path_out = PathBuf::from(path.to_string());
		let debug_out = PathBuf::from(debug.to_string());

		Self {
			name,
			out_path: path_out,
			debug_path: debug_out,
			..Default::default()
		}
	}
}

impl<'a> Executor for CssOperation<'a> {
	fn run(&mut self) -> Result<(), anyhow::Error> {

		println!("{}: start", self.name);

		let mut list_content = vec![];

		let temp_path = env::temp_dir();

		let mut out_system_debug_path = temp_path.clone();
		let mut out_system_path = temp_path;

		out_system_debug_path.push("out-debug.css");
		out_system_path.push("out.css");

		if out_system_debug_path.exists() {
			fs::remove_file(&out_system_debug_path)?;
		}
		
		let mut system_debug_file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(&out_system_debug_path)?;

		println!("{}: Concatenating files", self.name);

		for file_path in &self.files {

			let read_path = shellexpand::full(file_path)?;

			let result = fs::read(read_path.to_string())?;

			system_debug_file.write_all(&result)?;
		}

		println!("{}: Concatenating done", self.name);

		if out_system_path.exists() {
			fs::remove_file(&out_system_path)?;
		}

		let mut system_file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(&out_system_path)?;

		println!("{}: Processing files", self.name);

		for file_path in &self.files {

			let read_path = shellexpand::full(file_path)?;

			let result = fs::read(read_path.to_string())?;

			list_content.push(String::from_utf8_lossy(&result).to_string());
		}

		let temp = list_content.join("\n");

		let minified = Minifier::default().minify(&temp,
            Level::One
        ).unwrap();

		system_file.write_all(minified.as_bytes())?;

		if self.append {

			println!("{}: Appeding extra", self.name);
				
			for file_path in &self.extra {

				let read_path = shellexpand::full(file_path)?;

				let content = fs::read(read_path.to_string())?;

				system_debug_file.write_all(&content)?;
				system_file.write_all(&content)?;
			}

			println!("{}: Appeding done", self.name);
		}

		fs::rename(&out_system_debug_path, &self.debug_path)?;
		fs::rename(&out_system_path, &self.out_path)?;

		println!("{}: done", self.name);
		println!();

		Ok(())
	}
}
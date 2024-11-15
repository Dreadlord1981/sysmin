use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "Utility minify icecap / portfolio files")]
pub struct Arguments {
	
		
	#[arg(default_value="", help="List of files to minify")]
	pub list: String,

	#[arg(default_value="", help="List of files to minify")]
	pub extra: String,

	#[arg(short('i'), long("icecap"), default_value="false", help="minify icecap files")]
	pub icecap: bool,

	#[arg(short('p'), long("portfolio"), default_value="false", help="minify portfolio files")]
	pub portfolio: bool
}

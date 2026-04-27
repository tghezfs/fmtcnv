use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name="Input File. Accepted formats: json | toml | yaml | yml")]
    pub file: String,

    #[arg(short, long, value_name="Accepted values: json | toml | yaml | yml")]
    pub to_format: String,

    #[arg(short, long, value_name="Optional path or file for output")]
    pub out_file: Option<String>
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long)]
    pub to_format: String,

    #[arg(short, long)]
    pub out_file: String
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub day: u8,

    /// Number of times to greet
    #[arg(short, long)]
    pub part: u8,

    #[arg(short, long, default_value = None)]
    pub input_tag: Option<String>,
}

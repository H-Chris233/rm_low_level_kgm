use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "rm_low_level_kgm")]
#[command(author = "H-Chris233")]
#[command(version = "0.0.1")]
pub struct Cli {
    pub vec_dir: Vec<String>,
    #[arg(short, long)]
    pub traverse: bool,
}














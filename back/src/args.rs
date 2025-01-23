use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub populate: bool,
    #[arg(long, default_value_t = false)]
    pub download_training_data: bool,
}

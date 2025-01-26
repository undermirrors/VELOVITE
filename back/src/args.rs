use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub populate: bool,
    #[arg(long, default_value_t = false)]
    pub download_velov_data: bool,
    #[arg(short = 'm', long, default_value_t = 100)]
    pub max_velov_features: u32,
    #[arg(short = 's', long, default_value_t = 1)]
    pub velov_start: u32,
    #[arg(long, default_value_t = false)]
    pub download_weather_data: bool,

    #[arg(short, long, default_value_t = false)]
    pub filter_velov_data: bool,
    #[arg(short = 'M', long, default_value_t = false)]
    pub merge_datas: bool,
}

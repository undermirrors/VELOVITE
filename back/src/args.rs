use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Command-line arguments for the application.
///
/// # Fields
///
/// * `populate` - A flag to indicate whether to populate the database.
/// * `download_velov_data` - A flag to indicate whether to download Velov data.
/// * `max_velov_features` - The maximum number of Velov features to process.
/// * `velov_start` - The starting index for Velov data processing.
/// * `download_weather_data` - A flag to indicate whether to download weather data.
/// * `filter_velov_data` - A flag to indicate whether to filter Velov data.
/// * `merge_datas` - A flag to indicate whether to merge data sources.
/// * `mock` - A flag to indicate whether to use mock data.
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub populate: bool,
    #[arg(long, default_value_t = false)]
    pub download_velov_data: bool,
    #[arg(long, default_value_t = 100)]
    pub max_velov_features: u32,
    #[arg(short = 's', long, default_value_t = 1)]
    pub velov_start: u32,
    #[arg(long, default_value_t = false)]
    pub download_weather_data: bool,

    #[arg(short, long, default_value_t = false)]
    pub filter_velov_data: bool,
    #[arg(short = 'M', long, default_value_t = false)]
    pub merge_datas: bool,

    #[arg(short, long, default_value_t = false)]
    pub mock: bool,
    #[arg(short, long, default_value_t = false)]
    pub benchmark: bool,
}

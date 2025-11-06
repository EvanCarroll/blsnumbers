use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "blsnumbers",
    version,
    about = "Fetch and analyze BLS employment data for programmers and QA analysts",
    long_about = None
)]
pub struct Cli {
    /// BLS series ID(s) to fetch (e.g., CES6054150001 for Computer Systems Design industry)
    #[arg(short, long, value_delimiter = ',', default_value = "CES6054150001")]
    pub series: Vec<String>,

    /// Start year for data retrieval
    #[arg(long, default_value_t = 2022)]
    pub start_year: u16,

    /// End year for data retrieval
    #[arg(long, default_value_t = 2024)]
    pub end_year: u16,

    /// BLS API registration key (optional, can also use BLS_API_KEY env var)
    #[arg(long, env = "BLS_API_KEY")]
    pub api_key: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

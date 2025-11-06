mod api;
mod error;
mod analysis;
mod cli;

use clap::Parser;
use cli::{Cli, OutputFormat};
use api::BlsClient;
use analysis::{calculate_changes, format_change, format_percent};
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Create BLS client
    let client = BlsClient::new(args.api_key);

    // Fetch data
    println!("Fetching BLS data for series: {:?}", args.series);
    println!("Date range: {} - {}\n", args.start_year, args.end_year);

    let response = client
        .fetch_series(args.series.clone(), args.start_year, args.end_year)
        .await?;

    let results = response.results.ok_or_else(|| {
        error::BlsError::Api("No results in response".to_string())
    })?;

    // Process each series
    for series in results.series {
        println!("Series: {}", series.series_id);
        println!("{}", "=".repeat(80));

        let changes = calculate_changes(&series)?;

        match args.format {
            OutputFormat::Table => {
                print_table(&changes);
            }
            OutputFormat::Json => {
                print_json(&changes)?;
            }
        }

        println!("\n");
    }

    Ok(())
}

fn print_table(changes: &[analysis::EmploymentChange]) {
    // Print header
    println!(
        "{:<12} {:<15} {:>15} {:>15} {:>15}",
        "Year", "Month", "Employment (K)", "Change (K)", "% Change"
    );
    println!("{}", "-".repeat(80));

    // Print data rows
    for change in changes {
        println!(
            "{:<12} {:<15} {:>15.1} {:>15} {:>15}",
            change.year,
            change.month,
            change.value,
            format_change(change.change),
            format_percent(change.percent_change)
        );
    }
}

fn print_json(changes: &[analysis::EmploymentChange]) -> Result<()> {
    let json = serde_json::to_string_pretty(&changes)?;
    println!("{}", json);
    Ok(())
}

# BLS Numbers

A Rust CLI tool to fetch and analyze Bureau of Labor Statistics (BLS) employment data for programmers and related computer occupations.

## Overview

This tool retrieves monthly employment statistics from the BLS Current Employment Statistics (CES) program and displays month-over-month changes in employment levels.

## Important Note on Data

The BLS provides employment data through different programs:

- **CES (Current Employment Statistics)**: Provides **monthly** data by **industry** (NAICS codes), not specific occupations
- **CPS (Current Population Survey)**: Provides monthly data by occupation, but BLS recommends using annual averages due to sampling variability
- **OES (Occupational Employment Statistics)**: Provides **annual** data by specific occupation (e.g., Computer Programmers, QA Analysts)

This tool currently uses **CES data** which tracks the Computer Systems Design and Related Services industry (NAICS 541500). This includes programmers, software developers, QA analysts, and other computer professionals, but **does not separate them into individual occupation categories**.

## Installation

```bash
cargo build --release
```

## Usage

### Basic Usage

Fetch data with default settings (Computer Systems Design industry, 2022-2024):

```bash
cargo run --release
```

### Command Line Options

```
Options:
  -s, --series <SERIES>          BLS series ID(s) to fetch
                                 [default: CES6054150001]

      --start-year <START_YEAR>  Start year for data retrieval
                                 [default: 2022]

      --end-year <END_YEAR>      End year for data retrieval
                                 [default: 2024]

      --api-key <API_KEY>        BLS API registration key
                                 [env: BLS_API_KEY]

      --format <FORMAT>          Output format: table or json
                                 [default: table]

  -h, --help                     Print help
  -V, --version                  Print version
```

### Examples

#### Fetch specific years:

```bash
cargo run -- --start-year 2023 --end-year 2024
```

#### Output as JSON:

```bash
cargo run -- --format json
```

#### Use multiple series (with API key):

```bash
cargo run -- --series CES6054150001,CEU6054150001 --api-key YOUR_KEY
```

#### Use environment variable for API key:

```bash
export BLS_API_KEY=your_key_here
cargo run
```

## BLS API Key

While the tool works without an API key, registering for a free BLS API key provides higher rate limits:

- **Without key**: 25 queries/day, 10 years of data per request
- **With key**: 500 queries/day, 20 years of data per request

Register at: https://data.bls.gov/registrationEngine/

## Available Series

### CES Series (Monthly Industry Data)

- **CES6054150001**: All Employees, Computer Systems Design and Related Services (Seasonally Adjusted)
- **CEU6054150001**: All Employees, Computer Systems Design and Related Services (Not Seasonally Adjusted)

### Finding More Series

Browse available series at: https://www.bls.gov/ces/data/

## Output

The tool displays:

- **Year** and **Month** of the data point
- **Employment (K)**: Number of employed persons in thousands
- **Change (K)**: Month-over-month change in thousands
- **% Change**: Month-over-month percentage change

## Example Output

```
Fetching BLS data for series: ["CES6054150001"]
Date range: 2023 - 2024

Series: CES6054150001
================================================================================
Year         Month            Employment (K)      Change (K)        % Change
--------------------------------------------------------------------------------
2023         January                  2478.1             N/A             N/A
2023         February                 2480.9            +2.8          +0.11%
2023         March                    2481.1            +0.2          +0.01%
2023         April                    2482.2            +1.1          +0.04%
...
```

## Technical Details

- **Language**: Rust 2024 Edition
- **Dependencies**:
  - `clap`: Command-line argument parsing
  - `reqwest`: HTTP client for BLS API
  - `serde`/`serde_json`: JSON serialization
  - `tokio`: Async runtime
  - `thiserror`: Error handling

## License

This project is provided as-is for analyzing publicly available BLS employment statistics.

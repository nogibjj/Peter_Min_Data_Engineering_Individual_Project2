# Individual_Project_2


This is the README for my Individual Project 2 for the IDS706 - Data Engineering Systems class at Duke University.

## Overview
The purpose of this milestone project is to integrate a SQLite database with a Rust CLI tool while achieving the ETL-query pipeline functionalities. The dataset I used comes from FiveThirtyEight and it contains information on the location, employment, and distribution of librarians across the US.

## Usage
To use the tool, first ensure you have Rust and Cargo installed:

```
rustc --version
cargo --version
```

Then navigate to this project folder and run the calculator in 1 of 2 ways:

`cargo run -- calculate-total-balance <initial_deposit_amount> <monthly_contribution> <estimated_annual_growth_rate> <compounding_frequency> <number_of_years_to_accumulate>`

or
```
cargo build
./target/release/<package_name> calculate-total-balance <initial_deposit_amount> <monthly_contribution> <estimated_annual_growth_rate> <compounding_frequency> <number_of_years_to_accumulate>
```


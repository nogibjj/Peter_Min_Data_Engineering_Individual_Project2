# Mini_Project_7
[![CI/CD Pipeline](https://github.com/nogibjj/Peter_Min_Data_Engineering_Project7/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Peter_Min_Data_Engineering_Project7/actions/workflows/cicd.yml)

This is the README for my Mini Project 7 for the IDS706 - Data Engineering Systems class at Duke University.

## Overview
The purpose of this project is to translate my learning in Rust so far to a CLI tool. Since I am interested in achieving financial freedom as early as possible, and since one of the easiest way to do so is by investing early & compounding your return and/or interests, I will build a simple CLI tool that calculates the total investment balance from user inputs.

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

Assuming initial_deposit_amount = 5000, monthly_contribution = 200, estimated_annual_growth_rate = 0.1, compounding_frequency = 1, number_of_years_to_accumulate = 40, the output will be like this:
> Your total balance after 40 years with a initial deposit of $5000 and a monthly contribution of $200 under a annual growth rate of 10% with compounding frequency of 1 is $1288518.41.

## Note
- When entering the value for estimated annual growth rate, please enter it as a decimal value (e.g. if you estimate your wealth will grow by an average of 8% a year, enter 0.08). The output will display it as a regular interest rate format.
- When entering the value for compounding frequency, please reference this table: monthly = 12, quarterly = 4, semi-annually = 2, annually = 1, daily = 365.

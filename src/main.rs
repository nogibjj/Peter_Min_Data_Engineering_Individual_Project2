use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    CalculateTotalBalance {
        initial_deposit: f64,
        monthly_recurring_deposit: f64,
        annual_growth_rate: f64,
        compounding_frequency: i32,
        time_period: i32,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::CalculateTotalBalance {
            initial_deposit,
            monthly_recurring_deposit,
            annual_growth_rate,
            compounding_frequency,
            time_period,
        } => {
            let total_balance = peter_min_data_engineering_project7::calculate_total_balance(
                initial_deposit,
                monthly_recurring_deposit,
                annual_growth_rate,
                compounding_frequency,
                time_period,
            );
            let display_growth_rate = annual_growth_rate * 100.0;
            println!();
            println!("Your total balance after {} years with a initial deposit of ${} and a monthly contribution of ${} under a annual growth rate of {}% with compounding frequency of {} is ${:.4}.", 
            time_period,
            initial_deposit,
            monthly_recurring_deposit,
            display_growth_rate,
            compounding_frequency,
            total_balance);
            println!();
        }
    }
}

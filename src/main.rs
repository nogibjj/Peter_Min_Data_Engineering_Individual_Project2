use rust_cli_sqlite_librarian_data::{extract, load, create_record, update_record, delete_record, general_query};
use clap::{Parser, Subcommand};
use std::process;

/// Command-line arguments parser
#[derive(Parser)]
#[command(name = "Librarian Database SQLite Rust CLI")]
#[command(about = "ETL Pipeline for librarians dataset", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Extract,
    Load,
    UpdateRecord {
        state: String, 
        area: String, 
        total_employment: u64, 
        emp_prse: f64, 
        jobs_quotient_1000: f64, 
        location_quotient: f64
    },
    DeleteRecord {
        state: String,
    },
    CreateRecord {
        state: String, 
        area: String, 
        total_employment: u64, 
        emp_prse: f64, 
        jobs_quotient_1000: f64, 
        location_quotient: f64
    },
    GeneralQuery {
        query: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Actions::Extract => {
            println!("Extracting data: ");
            if let Err(e) = extract("https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/librarians/librarians-by-msa.csv", "data/librarians.csv") {
                eprintln!("Error occurred during Extract: {}", e);
                process::exit(1);
            }
        }
        Actions::Load => {
            println!("Loading & transforming data: ");
            if let Err(e) = load("data/librarians.csv") {
                eprintln!("Error occurred during TL: {}", e);
                process::exit(1);
            }
        }
        Actions::UpdateRecord {
            state,
            area,
            total_employment,
            emp_prse,
            jobs_quotient_1000, 
            location_quotient,
        } => {
            if let Err(e) = update_record(
                state,
                area,
                *total_employment,
                *emp_prse,
                *jobs_quotient_1000, 
                *location_quotient,
            ) {
                eprintln!("Error occurred during Update Record: {}", e);
                process::exit(1);
            }
        }
        Actions::DeleteRecord { state } => {
            if let Err(e) = delete_record(state) {
                eprintln!("Error occurred during Delete Record: {}", e);
                process::exit(1);
            }
        }
        Actions::CreateRecord {
            state,
            area,
            total_employment,
            emp_prse,
            jobs_quotient_1000, 
            location_quotient,
        } => {
            if let Err(e) = create_record(
                state,
                area,
                *total_employment,
                *emp_prse,
                *jobs_quotient_1000, 
                *location_quotient,
            ) {
                eprintln!("Error occurred during Create Record: {}", e);
                process::exit(1);
            }
        }
        Actions::GeneralQuery { query } => {
            match general_query(query) {
                Ok(data) => {
                    println!("Result of running your query: ");
                    for row in data {
                        println!("{:?}", row); // Use {:?} to print the tuple as Debug
                    }
                }
                Err(e) => {
                    eprintln!("Error occurred during querying: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
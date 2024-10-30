# Individual_Project_2
[![Rust CI/CD Pipeline](https://github.com/nogibjj/Peter_Min_Data_Engineering_Individual_Project2/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Peter_Min_Data_Engineering_Individual_Project2/actions/workflows/cicd.yml)

This is the README for my Individual Project 2 for the IDS706 - Data Engineering Systems class at Duke University.

## Overview
The purpose of this milestone project is to integrate a SQLite database with a Rust CLI tool while achieving the ETL-query pipeline functionalities. The dataset I used comes from FiveThirtyEight and it contains information on the location, employment, and distribution of librarians across the US. The software depends on several packages such as `clap`, `csv`, `reqwest`, and `rusqlite`.

## Usage
To use the tool, first ensure you have Rust and Cargo installed:

```
rustc --version
cargo --version
```

Then navigate to this project folder and run the CLI commands in 1 of 2 ways:

`cargo run -- <operation> <arguments>`

or
`cargo build && ./target/release/<package_name> <operation> <arguments>`

The following commands are supported:
- Extract: `cargo run -- extract`
- Load-Transform: `cargo run -- load`
- Create: `cargo run -- create-record ` followed by 6 arguments of type (String, String, positive integer, float, float, float).
- Update: `cargo run -- update-record ` followed by 6 arguments of type (String, String, positive integer, float, float, float).
- Delete: `cargo run -- delete-record ` followed by 1 String argument.
- Read (or a general query): `cargo run -- general-query ` followed by a valid SQL query string.

## Use of LLM
The syntax and learning curve of Rust as a programming language for me who comes from a primarily scripting-based background (e.g. Python and JavaScript) is pretty challenging, so I applied LLM to help me debug issues around crates, modules import, debugging cargo commands, and write test cases for me throughout this project.

## Demo Video
Here is the [link](https://youtu.be/_JkRfR5ID0k) to my demo video on YouTube.
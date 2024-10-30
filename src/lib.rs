use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::{self, BufReader, Write};

type LibrarianData = Vec<LibrarianRecord>;
type LibrarianRecord = (String, String, u64, f64, f64, f64);


pub fn extract(url: &str, file_path: &str) -> io::Result<()> {
    let response = get(url).expect("Failed to fetch data from URL.");
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes().expect("Failed to read bytes."))?;
    Ok(())
}

pub fn load(file_path: &str) -> Result<()> {
    let conn = Connection::open("Librarians.db")?;
    conn.execute("DROP TABLE IF EXISTS Librarians", [])?;
    conn.execute("CREATE TABLE Librarians (
        state TEXT,
        area TEXT,
        total_employment INTEGER,
        emp_prse REAL,
        jobs_quotient_1000 REAL,
        location_quotient REAL
    )", [])?;

    let file = File::open(file_path).expect("Failed to open CSV file.");
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
    let mut stmt = conn.prepare("INSERT INTO Librarians (state, area, total_employment, emp_prse, jobs_quotient_1000, location_quotient) VALUES (?, ?, ?, ?, ?, ?)")?;

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        stmt.execute(params![&record[0], &record[1],
            record[2].parse::<u64>().unwrap_or(0),
            record[3].parse::<f64>().unwrap_or(0.0),
            record[4].parse::<f64>().unwrap_or(0.0),
            record[5].parse::<f64>().unwrap_or(0.0),
        ])?;
    }
    Ok(())
}

pub fn create_record(
    state: &str, 
    area: &str, 
    total_employment: u64, 
    emp_prse: f64, 
    jobs_quotient_1000: f64, 
    location_quotient: f64) -> Result<()> {
    let conn = Connection::open("Librarians.db")?;
    conn.execute(
        "INSERT INTO Librarians (state, area, total_employment, emp_prse, jobs_quotient_1000, location_quotient) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&state, &area, total_employment, emp_prse, jobs_quotient_1000, location_quotient])?;
    Ok(())
}

pub fn update_record(
    state: &str, 
    area: &str, 
    total_employment: u64, 
    emp_prse: f64, 
    jobs_quotient_1000: f64, 
    location_quotient: f64) -> Result<()> {
    let conn = Connection::open("Librarians.db")?;
    conn.execute(
        "UPDATE Librarians SET area=2, total_employment=?3, emp_prse=?4, jobs_quotient_1000=?5, location_quotient=?6 WHERE state = ?1",
        params![state, area, total_employment, emp_prse, jobs_quotient_1000, location_quotient])?;
    Ok(())
}

pub fn delete_record(state: &str) -> Result<()> {
    let conn = Connection::open("Librarians.db")?;
    conn.execute("DELTE FROM Librarians WHERE state=?", params![state])?;
    Ok(())
}

pub fn general_query(query: &str) -> Result<LibrarianData> {
    let conn = Connection::open("Librarians.db")?;
    let mut results = Vec::new();

    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?
            ))
        })?;

        for row in rows {
            results.push(row?);
        }
    } else {
        conn.execute(query, [])?;
    }
    Ok(results)
}
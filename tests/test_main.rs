use std::process::Command;
const TEST_DB: &str = "Librarians.db";


#[test]
fn test_extract() {
    let result = Command::new("cargo")
        .args(&["run", "--", "extract"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute extract command");

    println!("Output from extract: {:?}", result);
    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Extracting data: "));
}

#[test]
fn test_load() {
    let result = Command::new("cargo")
        .args(&["run", "--", "load"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to load database");

    println!("Output from load: {:?}", result);
    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Loading & transforming data: "));
}

#[test]
fn test_create_record() {
    let result = Command::new("cargo")
    .args(&[
        "run",
        "--",
        "create-record",
        "NC",
        "Raleigh-Durham-ChapelHill",
        "20",
        "10.8",
        "12.5",
        "3.4"
    ])
    .env("DATABASE_URL", TEST_DB)
    .output()
    .expect("Failed to create record.");

    println!("Output from create-record: {:?}", result);
    assert!(result.status.success());
}

#[test]
fn test_update_record() {
    let result = Command::new("cargo")
    .args(&[
        "run",
        "--",
        "update-record",
        "NC",
        "Raleigh-Durham-ChapelHill",
        "15",
        "20.8",
        "2.70",
        "3.2"
    ])
    .env("DATABASE_URL", TEST_DB)
    .output()
    .expect("Failed to execute create_record command");

    println!("Output from create-record: {:?}", result);
    assert!(result.status.success());
}

#[test]
fn test_delete_record() {
    let result = Command::new("cargo")
    .args(&[
        "run",
        "--",
        "delete-record",
        "Raleigh-Durham-ChapelHill"
    ])
    .env("DATABASE_URL", TEST_DB)
    .output()
    .expect("Failed to execute create_record command");

    println!("Output from create-record: {:?}", result);
    assert!(result.status.success());
}

#[test]
fn test_get_record() {
    let result = Command::new("cargo")
    .args(&[
        "run",
        "--",
        "general-query",
        "SELECT * FROM Librarians LIMIT 10;"
    ])
    .env("DATABASE_URL", TEST_DB)
    .output()
    .expect("Failed to execute create_record command");

    println!("Output from create-record: {:?}", result);
    assert!(result.status.success());
}
use chrono::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

pub fn save(data: String, path: String) -> std::io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let formatted_time = local.format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = format!("{path}/{}_backup.hex", formatted_time);
    let mut file = File::create(file_name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn filter(dfx_stdout: &str) -> Result<String, String> {
    let re = Regex::new(r#"blob "([^"]*)""#).unwrap();

    if let Some(captures) = re.captures(dfx_stdout) {
        if let Some(matched) = captures.get(1) {
            Ok(format!("{}", matched.as_str()))
        } else {
            Err(format!("ERROR"))
        }
    } else {
        Err(format!("ERROR"))
    }
}

pub fn read(path: String) -> String {
    let mut file = File::open(path).expect("msg");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    contents
}

pub fn run(args: Vec<&str>) -> String {
    // Command to be executed
    let command = "dfx"; // Replace with the desired command

    // Execute the command
    let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the output to a string and print it
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    } else {
        // Convert the error output to a string and print it
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("Error: {}", stderr)
    }
}

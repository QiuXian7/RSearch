use chrono::Local;
use std::fs::OpenOptions;
use std::io::{Read, Write};

const HISTORY_FILE: &str = "history.log";

pub fn add_history(command: &str) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S");

    let record = format!("[{}] {}\n", time, command);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)
        .unwrap();

    file.write_all(record.as_bytes()).unwrap();
}

pub fn show_history() {
    let mut content = String::new();
    println!("\n搜索记录:");
    if let Ok(mut file) = OpenOptions::new().read(true).open(HISTORY_FILE) {
        file.read_to_string(&mut content).unwrap();

        println!("{}", content);
    } else {
        println!("暂无搜索记录");
    }
}

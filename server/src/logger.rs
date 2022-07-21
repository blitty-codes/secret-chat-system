use std::{fs, env, path::PathBuf, io::{self, Write}};
use chrono;

#[derive(Debug, Clone)]
pub struct Logger {
    name: String,
    path: PathBuf
}

pub fn create_log(name: String) -> Result<Logger, io::Error> {
    let name = format!("{}.log", name.clone());
    let _file = fs::File::create(name.clone()).unwrap();
    let path = env::current_dir()?;
    let path = path.join(name.clone());

    Ok(Logger {
        name,
        path
    })
}

pub fn write_log(logger: &Logger, msg: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&logger.path)
        .unwrap();

    let content = format!("[*] {} - {}\n----------\n", chrono::offset::Utc::now(), msg);
    file.write_all(content.as_bytes()).unwrap();
}

pub fn write_issue_log(logger: &Logger, msg: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&logger.path)
        .unwrap();

    let content = format!("[-] {} - {}\n----------\n", chrono::offset::Utc::now(), msg);
    file.write_all(content.as_bytes()).unwrap();
}

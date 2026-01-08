use std::{
    env,
    fs::{self, OpenOptions, read_to_string},
    io::{Error, ErrorKind, Read, Write},
    path::PathBuf,
};

use rbonsai::bonsai::TreeConfig;
use serde::{Deserialize, Serialize};

const STORAGE_FILE: &str = "trees";

#[derive(Serialize, Deserialize)]
pub struct TreeStore {
    pub config: TreeConfig,
    pub seed: u64,
    pub withered: bool,
}

fn open_storage_dir() -> Result<PathBuf, Error> {
    // deprecated because may have unexpected behavior on Windows; Windows not supported anyway
    #[allow(deprecated)]
    let storage_dir = match env::home_dir() {
        Some(dir) => dir,
        None => return Err(Error::new(ErrorKind::NotFound, "Home directory not found")),
    }
    .join(r".bonsai-rs");
    fs::create_dir_all(&storage_dir)?;
    Ok(storage_dir)
}

fn open_storage_file() -> Result<PathBuf, Error> {
    Ok(open_storage_dir()?.join(STORAGE_FILE))
}

pub fn add_tree(store: TreeStore) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(open_storage_file()?)?;
    writeln!(&mut file)?;
    file.write_all(serde_json::to_string(&store)?.as_bytes())
}

pub fn get_trees() -> Result<Vec<TreeStore>, Error> {
    let file_path = open_storage_file().expect("Failed to construct the file path");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file_path)
        .expect("Failed to open/create storage file");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Failed to read storage file to buffer");

    let mut result: Vec<TreeStore> = Vec::new();
    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }
        result.push(serde_json::from_str(line)?);
    }
    Ok(result)
}

use std::path::PathBuf;

pub fn file_path() -> PathBuf {
    let mut path = dirs::config_local_dir().unwrap();
    path.push("Planer");
    path
}
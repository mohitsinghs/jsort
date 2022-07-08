use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
};

pub fn ensure(path: &Path) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("failed to write")
}

pub fn with_suffix(suffix: &str, path: &Path) -> PathBuf {
    let mut suffixed = path.file_stem().unwrap().to_os_string();
    suffixed.push(format!("-{}.", suffix));
    suffixed.push(path.extension().unwrap());
    PathBuf::from(suffixed)
}

pub fn with_dir(dir: &str, path: &Path) -> PathBuf {
    create_dir_all(dir).expect("failed to create output dir");
    Path::new(dir).join(path).to_path_buf()
}

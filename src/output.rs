use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::Path,
};

pub fn with_replace(path: &Path) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("failed to write")
}

pub fn with_suffix(suffix: &str, path: &Path) -> File {
    let mut suffixed = path.file_stem().unwrap().to_os_string();
    suffixed.push(format!("-{}.", suffix));
    suffixed.push(path.extension().unwrap());
    with_replace(Path::new(&suffixed))
}

pub fn with_dir(dir: &str, path: &Path) -> File {
    create_dir_all(dir).expect("failed to create output dir");
    let output = Path::new(dir).join(path);
    with_replace(output.as_path())
}

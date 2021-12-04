use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn execute(path: &PathBuf) {
    info!("git init {:?}", path);
    init_dirs(&Path::new(&path).join(".git-rs"));
}

/// ignore file already exists error
/// TODO: handle other error, such as permission
#[allow(unused_must_use)]
fn init_dirs(base_dir: &Path) {
    fs::create_dir(base_dir);

    let info = base_dir.join("info");
    let objects = base_dir.join("objects");
    let refs = base_dir.join("refs");
    fs::create_dir(&info);
    fs::create_dir(&objects);
    fs::create_dir(&refs);

    fs::File::create(&info.join("exclude"));
    fs::create_dir(&objects.join("info"));
    fs::create_dir(&refs.join("heads"));
    fs::create_dir(&refs.join("tags"));
}

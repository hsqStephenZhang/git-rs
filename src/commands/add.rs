use std::path::PathBuf;

pub fn execute(files: &Vec<PathBuf>) {
    info!("git add files:{:?}", files);
}

pub fn add_one(_file: &PathBuf) {}

#[cfg(test)]
mod tests {
    use walkdir::WalkDir;

    #[test]
    fn test_walkdir() {
        for entry in WalkDir::new("data").max_depth(1) {
            println!("{}", entry.unwrap().path().display());
        }
    }
}

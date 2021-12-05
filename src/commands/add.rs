use std::path::PathBuf;

/**
 * 1. read the cache(if exists) and parse
 * 2. if the file we created is normal file(not dir), do the following things:
 *      i.     create the blob object
 *      ii.    create tree object if necessary
 *      iii.   create 
 * 3. update the cache(or create the new one)
 * 
 */ 

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

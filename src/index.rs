use std::fs;
use std::os::linux::fs::MetadataExt;
use std::os::unix::prelude::OsStringExt;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Index {
    pub desc: u32,
    pub version: u32,
    pub num_entrys: u32,
    pub entrys: Vec<IndexEntry>,
    pub checksum: Vec<u8>,
    pub extensions: Option<Vec<Extension>>,
}

impl Index {
    pub fn new(
        desc: u32,
        version: u32,
        num_entrys: u32,
        entrys: Vec<IndexEntry>,
        checksum: Vec<u8>,
        extensions: Option<Vec<Extension>>,
    ) -> Self {
        Self {
            desc,
            version,
            num_entrys,
            entrys,
            checksum,
            extensions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexEntry {
    pub ctime: i64,
    pub mtime: i64,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub filesize: u32,
    pub sha1: Vec<u8>,
    pub flags: u16,
    pub filepath: Vec<u8>,
    pub padding: usize,
}

#[allow(unused_variables)]
impl IndexEntry {
    pub fn new(
        ctime: i64,
        mtime: i64,
        dev: u32,
        ino: u32,
        mode: u32,
        uid: u32,
        gid: u32,
        filesize: u32,
        sha1: Vec<u8>,
        flags: u16,
        filepath: Vec<u8>,
        padding: usize,
    ) -> IndexEntry {
        Self {
            ctime,
            mtime,
            dev,
            ino,
            mode,
            uid,
            gid,
            filesize,
            sha1,
            flags,
            filepath,
            padding,
        }
    }

    pub fn try_new(path: PathBuf) -> std::io::Result<Self> {
        let meta = fs::metadata(path.clone())?;
        let ctime = meta.st_ctime();
        let mtime = meta.st_mtime();
        let dev = meta.st_dev() as u32;
        let ino = meta.st_ino() as u32;
        let mode = meta.st_mode();
        let uid = meta.st_uid();
        let gid = meta.st_gid();
        let filesize = meta.st_size() as u32;

        // let sha1 = crate::SHA_CACHE.lock().unwrap();
        // let cached = sha1.get(&path);
        // let mut sha1 = match cached {
        //     Some(cached) => cached.clone(),
        //     None => {
        //         let bytes = std::fs::read(&path).unwrap();
        //         bytes
        //     }
        // };
        // FIXME: should get the sha1 not the raw bytes
        let mut sha1 = std::fs::read(&path).unwrap();
        sha1.push(b'\0');
        let filepath = path.into_os_string().into_vec();
        // FIXME: flags should be split into high and low bits, we ignore high bits here
        let flags = if filepath.len() >= 0xFFF {
            0xFFF
        } else {
            filepath.len() as u16
        };
        let mut total_size = 8 * 2 + 4 * 6;
        total_size += sha1.len();
        total_size += filepath.len();
        total_size += 2;

        let padding = total_size % 8;

        Ok(Self {
            ctime,
            mtime,
            dev,
            ino,
            mode,
            uid,
            gid,
            filesize,
            sha1,
            flags,
            filepath,
            padding,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Extension {
    Tree(Tree),
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub entry_count: usize,
    pub subtree_count: usize,
    pub sha1: Vec<u8>,
    pub children: Option<Vec<Tree>>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn t1() {}
}

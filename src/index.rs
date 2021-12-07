use std::collections::HashMap;
use std::fs;
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
use std::path::PathBuf;

use crate::core::working_area::WorkingArea;

#[derive(Clone, Debug)]
pub struct Index {
    pub desc: u32,
    pub version: u32,
    pub num_entrys: u32,
    pub entrys: Vec<IndexEntry>,
    pub tree_extension: Option<TreeExtension>, // recursive, layer first structure
    pub checksum: Vec<u8>,
}

impl Index {
    pub fn new(
        desc: u32,
        version: u32,
        num_entrys: u32,
        entrys: Vec<IndexEntry>,
        checksum: Vec<u8>,
        tree_extension: Option<TreeExtension>,
    ) -> Self {
        Self {
            desc,
            version,
            num_entrys,
            entrys,
            checksum,
            tree_extension,
        }
    }

    pub fn entry_map(&self) -> HashMap<PathBuf, Vec<u8>> {
        self.entrys
            .iter()
            .map(|v| {
                let path: String = v.filepath.clone();
                (PathBuf::from(path), v.sha1.clone())
            })
            .collect()
    }

    pub fn compare_working_area(&self, _working_area: &WorkingArea) {
        todo!()
    }

    pub fn invalid_nodes(&self) {
        todo!()
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
    pub filepath: String,
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
        filepath: String,
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

        // FIXME: should get the sha1 not the raw bytes
        let mut sha1 = std::fs::read(&path).unwrap();
        sha1.push(b'\0');
        let filepath:String = path.into_os_string().into_string().unwrap();
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
pub struct TreeExtension {
    pub path: String,
    pub entry_num: i32,
    pub subtree_num: i32,
    pub sha1: Option<Vec<u8>>,
    pub children: Vec<TreeExtension>,
}

impl TreeExtension {
    pub fn new(
        path: String,
        entry_num: i32,
        subtree_num: i32,
        sha1: Option<Vec<u8>>,
        children: Vec<TreeExtension>,
    ) -> Self {
        Self {
            path,
            entry_num,
            subtree_num,
            sha1,
            children,
        }
    }
}

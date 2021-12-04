use bytes::{BufMut, BytesMut};

/// commit <content length><NUL>tree <tree sha>
/// parent <parent sha>
/// [parent <parent sha> if several parents from merges]
/// author <author name> <author e-mail> <timestamp> <timezone>
/// committer <author name> <author e-mail> <timestamp> <timezone>
///
/// <commit message>
#[derive(Clone, Debug)]
pub struct Commit {
    pub root_sha1: String,
    pub parents_sha1: Option<Vec<String>>,
    pub author: Option<AuthorInfo>,
    pub commiter: Option<CommitterInfo>,
    pub messsage: String,
}

impl Commit {
    pub fn new(
        root_sha1: String,
        parents_sha1: Option<Vec<String>>,
        author: Option<AuthorInfo>,
        commiter: Option<CommitterInfo>,
        messsage: String,
    ) -> Self {
        Self {
            root_sha1,
            parents_sha1,
            author,
            commiter,
            messsage,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthorInfo {
    pub name: String,
    pub email: String,
    pub timestamp: usize,
    pub time_zone: String,
}

#[derive(Clone, Debug)]
pub struct CommitterInfo {
    pub name: String,
    pub email: String,
    pub timestamp: usize,
    pub time_zone: String,
}

impl AuthorInfo {
    pub fn new(name: String, email: String, timestamp: usize, time_zone: String) -> Self {
        Self {
            name,
            email,
            timestamp,
            time_zone,
        }
    }
}

impl CommitterInfo {
    pub fn new(name: String, email: String, timestamp: usize, time_zone: String) -> Self {
        Self {
            name,
            email,
            timestamp,
            time_zone,
        }
    }
}

impl Into<Vec<u8>> for &Commit {
    fn into(self) -> Vec<u8> {
        let mut entry_buf = BytesMut::with_capacity(2048);

        entry_buf.put(&b"tree "[..]);
        entry_buf.put(self.root_sha1.as_bytes());
        entry_buf.put_u8(b'\n');

        // dbg!(entry_buf.split());

        // println!("{:?}\n\n",&entry_buf[..]);

        if let Some(ref parents) = self.parents_sha1 {
            for entry in parents.iter() {
                entry_buf.put(&b"parent "[..]);
                entry_buf.put(entry.as_bytes());
                entry_buf.put_u8(b'\n');
            }
        }

        // dbg!(entry_buf.split());

        // println!("{:?}\n\n",&entry_buf[..]);

        if let Some(ref author_info)=self.author{
            entry_buf.put(&b"author "[..]);
            entry_buf.put(author_info.name.as_bytes());
            entry_buf.put_u8(b' ');
            entry_buf.put(author_info.email.as_bytes());
            entry_buf.put_u8(b' ');
            let timestamp=format!("{}",author_info.timestamp);
            entry_buf.put(timestamp.as_bytes());
            entry_buf.put_u8(b' ');
            entry_buf.put(author_info.time_zone.as_bytes());
            entry_buf.put_u8(b'\n');
        }

        // dbg!(entry_buf.split());

        if let Some(ref committer_into)=self.author{
            entry_buf.put(&b"committer "[..]);
            entry_buf.put(committer_into.name.as_bytes());
            entry_buf.put_u8(b' ');
            let timestamp=format!("{}",committer_into.timestamp);
            entry_buf.put(committer_into.email.as_bytes());
            entry_buf.put_u8(b' ');
            entry_buf.put(timestamp.as_bytes());
            entry_buf.put_u8(b' ');
            entry_buf.put(committer_into.time_zone.as_bytes());
            entry_buf.put_u8(b'\n');
        }

        // dbg!(entry_buf.split());
        entry_buf.put_u8(b'\n');
        entry_buf.put(self.messsage.as_bytes());

        // dbg!(entry_buf.split());

        let entry_contents = &entry_buf[..];
        let length = entry_contents.len();
        let s = format!("{}", length);
        let mut buf = BytesMut::with_capacity(length + 6 + 10);
        buf.put(&b"commit "[..]);
        buf.put(s.as_bytes());
        buf.put_u8(b'\0');
        buf.put(entry_contents);
        (&buf[..]).into()
        // Vec::new()
    }
}

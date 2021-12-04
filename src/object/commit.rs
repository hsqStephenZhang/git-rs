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

pub mod add;
pub mod cat;
pub mod commit;
pub mod init;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "the stupid content tracker")]
pub enum Git {
    Init {
        #[structopt(short, long, default_value = ".")]
        path: PathBuf,
    },
    Add {
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Fetch {
        #[structopt(short, long)]
        dry_run: bool,
        #[structopt(short, long)]
        all: bool,
        repository: Option<String>,
    },
    Commit {
        #[structopt(short, long)]
        message: Option<String>,
        #[structopt(short, long)]
        all: bool,
    },
    Cat {
        mode: String,
        files: Vec<PathBuf>,
    },
}

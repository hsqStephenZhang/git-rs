pub mod commands;
pub mod db;
pub mod diff;
pub mod error;
pub mod filecache;
pub mod index;
pub mod object;
pub mod parser;
pub mod tree;
pub mod utils;

pub use filecache::*;
use commands::Git;
use structopt::StructOpt;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

#[macro_use]
extern crate lazy_static;

pub const APP_NAME: &'static str = "git-rs";
pub const REPO_NAME: &'static str = ".git-rs";

fn main() {
    init();

    let exec = Git::from_args();
    match exec {
        Git::Init { path } => {
            commands::init::execute(&path);
        }
        Git::Add { files } => {
            commands::add::execute(&files);
        }
        _ => {}
    }
}

fn init() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .expect("init simple log failed");
}

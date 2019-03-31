use quicli::prelude::*;
use structopt::StructOpt;
use std::process::Command;

use std::io;

use git2::Repository;
use git2::RepositoryOpenFlags;

use failure::Fail;
 
use derive_more::From as DeriveFrom;

#[derive(Debug, StructOpt)]
struct Cli {
    cmd: String,

    #[structopt(long = "search", short = "s", default_value= "")]
    search: String,
}

const RUST_DOC_HTTP: &str = "https://doc.rust-lang.org/std/index.html?search=";

#[derive(Debug, DeriveFrom, Fail)]
pub enum Error {
    #[fail(display = "git error - {}", _0)]
    Git2Error(#[cause] git2::Error),
    
    #[fail(display = "No origin url found")]
    OriginUrlNotFound,

    #[fail(display = "command error: {}", _0)]
    IoError(#[cause] io::Error),
}

fn get_local_github_url() -> Result<String, Error> {
    let repo = Repository::open_ext(".", RepositoryOpenFlags::empty(), vec!["/home"])?;
    let origin = repo.find_remote("origin")?;
    let origin_git_url = origin.url().ok_or(Error::OriginUrlNotFound)?;
    let origin_https = origin_git_url.replace(".git", "").replace(":", "/").replace("git@", "https://");
    return Ok(origin_https);
}

fn travis() -> Result<(), Error>{
    let github_url = get_local_github_url()?;
    let travis_url = github_url.replace("github.com", "travis-ci.org");
    let mut process = Command::new("xdg-open").arg(travis_url).spawn().unwrap();
    process.wait().expect("waiting for command to finish");
    println!("travis has opened sucessfully");

    Ok(())
}

fn github() -> Result<(), Error> {
    let github_url = get_local_github_url()?;
    let mut process = Command::new("xdg-open").arg(github_url).spawn()?;
    process.wait().expect("waiting for command to finish");
    println!("github has opened sucessfully");

    Ok(())
}

fn rust(doc_search: String) -> Result<(), Error> {
    
    let rust_doc_url = format!("{}{}", RUST_DOC_HTTP, doc_search);
    let mut process = Command::new("xdg-open").arg(rust_doc_url).spawn()?;
    process.wait().expect("waiting for command to finish");
    println!("github has opened sucessfully");

    Ok(())
}

fn main() -> CliResult {
    let args = Cli::from_args();
    match args.cmd.as_ref() {
        "github" => github()?,
        "travis" => travis()?,
        "rust" => rust(args.search)?,
        _ => println!("Not supported command"),
    }

    Ok(())
}

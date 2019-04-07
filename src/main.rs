use quicli::prelude::*;
use structopt::StructOpt;
use std::process::Command;

use std::io;
use std::collections::HashMap;

use git2::Repository;
use git2::RepositoryOpenFlags;

use failure::Fail;

use serde_json;
use std::path::Path;
use std::fs::File;
 
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

    #[fail(display = "No homedir could be found")]
    HomeDirNotFound,

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

#[derive(Debug, Deserialize)]
pub struct UrlsMap{
    pub url_map: HashMap<String, String>,
}

fn url(url_key: &str) -> Result<(), Error> {
    println!("looking for url for key: {}", url_key);
    let homedir = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
    let homedir_str = homedir.to_str().ok_or(Error::HomeDirNotFound)?;
    let file_raw_path = format!("{}{}", homedir_str, "/.config/goto/urls.json");
    let json_file_path = Path::new(&file_raw_path);
    let json_file = File::open(json_file_path).expect("urls file not found in ~/.config/goto/urls.json");
    let url_map: UrlsMap = serde_json::from_reader(json_file).expect("error while reading json");
    if url_map.url_map.contains_key(url_key) {
        let url = &url_map.url_map[url_key];
        println!("found {} opening {}", url_key, url);
        let mut process = Command::new("xdg-open").arg(url).spawn()?;
        process.wait().expect("waiting for command to finish");
        println!("url opened");
    } else {
        println!("url not found for: {}", url_key);
        println!("add new url at ~/.config/goto/urls.json");
    }

    Ok(())
}

fn main() -> CliResult {

    let args = Cli::from_args();
    match args.cmd.as_ref() {
        "github" => github()?,
        "travis" => travis()?,
        "rust" => rust(args.search)?,
        _ => url(&args.cmd)?,
    }

    Ok(())
}

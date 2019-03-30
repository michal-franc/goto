use quicli::prelude::*;
use structopt::StructOpt;
use std::process::Command;
use git2::Repository;
use git2::RepositoryOpenFlags;

#[derive(Debug, StructOpt)]
struct Cli {
    cmd: String,
}

fn get_local_github_url() -> String {

    let v = vec!["/home"];

    let repo = match Repository::open_ext(".", RepositoryOpenFlags::empty(), v) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let origin = match repo.find_remote("origin") {
        Ok(origin) => origin,
        Err(e) => panic!("remote origin not found: {}", e),
    };

    let origin_git_url = match origin.url() {
        Some(url) => url,
        None => panic!("No origin url found"),
    };

    println!("found remote origin: {:?}", origin_git_url);

    let origin_https = origin_git_url.replace(".git", "").replace(":", "/").replace("git@", "https://");
    return origin_https;
}

fn travis() {
    let travis_url = get_local_github_url().replace("github.com", "travis-ci.org");
    Command::new("xdg-open").arg(travis_url).spawn();
}

fn github() {
    Command::new("xdg-open").arg(get_local_github_url()).spawn();
}

fn main() -> CliResult {
    let args = Cli::from_args();
    match args.cmd.as_ref() {
        "github" => github(),
        "travis" => travis(),
        _ => println!("Not supported command"),
    }

    Ok(())
}

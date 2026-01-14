use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::Command;
use clap::{Parser, Subcommand};
use thiserror::Error;

use git2::Repository;
use git2::RepositoryOpenFlags;

#[derive(Debug, Parser)]
#[command(
    name = "goto",
    about = "Quick navigation to web resources from your terminal.\n\n\
             Run from a git repository to open GitHub, Travis CI, or use custom URL shortcuts."
)]
enum Cli {
    #[command(
        name = "github",
        about = "Open the GitHub page for the current repository",
        long_about = "Open the GitHub page for the current repository.\n\n\
                      Run this from any directory inside a git repo with a GitHub remote.\n\
                      The tool reads the 'origin' remote URL and opens it in your browser."
    )]
    Github {
        #[arg(short = 'c', long = "commit", help = "Open a specific commit by hash")]
        commit: Option<String>,
    },

    #[command(
        name = "travis",
        about = "Open the Travis CI page for the current repository"
    )]
    Travis,

    #[command(
        name = "rust",
        about = "Search the Rust standard library documentation"
    )]
    Rust {
        #[arg(
            short = 's',
            long = "search",
            help = "Search term to look up in Rust docs"
        )]
        search: String,
    },

    #[command(
        name = "url",
        about = "Open a custom URL from your config file",
        long_about = "Open a custom URL from your config file.\n\n\
                      URLs are stored in ~/.config/goto/urls.json\n\n\
                      Run without arguments to see available keys.\n\n\
                      Example config:\n\
                      {\n  \
                        \"url_map\": {\n    \
                          \"gh\": \"https://github.com\",\n    \
                          \"docs\": \"https://docs.rs\"\n  \
                        }\n\
                      }"
    )]
    Url {
        #[arg(help = "The key name for the URL (omit to list available keys)")]
        key: Option<String>,
    },

    #[command(name = "config", about = "Manage goto configuration")]
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCmd {
    #[command(name = "url", about = "Add or update a URL shortcut")]
    Url {
        #[arg(help = "The key name for the URL")]
        key: String,
        #[arg(help = "The URL to open")]
        url: String,
    },
}

const RUST_DOC_HTTP: &str = "https://doc.rust-lang.org/std/index.html?search=";

#[derive(Debug, Error)]
pub enum Error {
    #[error("git error - {0}")]
    Git2Error(#[from] git2::Error),

    #[error("No origin url found")]
    OriginUrlNotFound,

    #[error("Origin format not supported")]
    OriginFormatNotSupported,

    #[error("Unable to acquire github url")]
    ErrorWhenAcquiringUrl,

    #[error("No homedir could be found")]
    HomeDirNotFound,

    #[error("command error: {0}")]
    IoError(#[from] io::Error),
}

/// Converts a git origin URL to a GitHub HTTPS URL
/// Supports SSH format: git@github.com:user/repo.git
/// Supports HTTPS format: https://github.com/user/repo.git
fn parse_git_origin_to_github_url(origin_url: &str) -> Result<String, Error> {
    // Handle SSH format: git@github.com:user/repo.git
    if origin_url.starts_with("git@") {
        let find_index = origin_url
            .find(':')
            .ok_or(Error::OriginFormatNotSupported)?;
        let path = origin_url[find_index + 1..].replace(".git", "");
        return Ok(format!("https://github.com/{}", path));
    }

    // Handle HTTPS format: https://github.com/user/repo.git
    if origin_url.starts_with("https://") || origin_url.starts_with("http://") {
        let url = origin_url.replace(".git", "");
        return Ok(url);
    }

    Err(Error::OriginFormatNotSupported)
}

fn get_local_github_url() -> Result<String, Error> {
    let repo = Repository::open_ext(".", RepositoryOpenFlags::empty(), vec!["/home"])?;
    let origin = repo.find_remote("origin")?;
    let origin_git_url = origin.url().ok_or(Error::OriginUrlNotFound)?;
    parse_git_origin_to_github_url(origin_git_url)
}

fn get_local_github_url_with_commit(commit_hash: String) -> Result<String, Error> {
    let mut github_url = get_local_github_url()?;
    github_url.push_str("/commit/");
    github_url.push_str(&commit_hash);
    Ok(github_url)
}

fn travis() -> Result<(), Error> {
    let github_url = get_local_github_url()?;
    let travis_url = github_url.replace("github.com", "travis-ci.org");
    let mut process = Command::new("xdg-open").arg(travis_url).spawn().unwrap();
    process.wait().expect("waiting for command to finish");
    println!("travis has opened sucessfully");

    Ok(())
}

fn github(commit: Option<String>) -> Result<(), Error> {
    let github_url = match commit {
        Some(hash) => get_local_github_url_with_commit(hash)?,
        None => get_local_github_url()?,
    };

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

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlsMap {
    pub url_map: HashMap<String, String>,
}

fn get_config_path() -> Result<String, Error> {
    let homedir = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
    let homedir_str = homedir.to_str().ok_or(Error::HomeDirNotFound)?;
    Ok(format!("{}/.config/goto/urls.json", homedir_str))
}

fn load_url_config() -> Result<UrlsMap, Error> {
    let file_path = get_config_path()?;
    let json_file_path = Path::new(&file_path);

    if !json_file_path.exists() {
        return Ok(UrlsMap {
            url_map: HashMap::new(),
        });
    }

    let json_file = File::open(json_file_path).expect("failed to open urls.json");
    let url_map: UrlsMap = serde_json::from_reader(json_file).expect("error while reading json");
    Ok(url_map)
}

fn save_url_config(config: &UrlsMap) -> Result<(), Error> {
    let file_path = get_config_path()?;
    let json_file_path = Path::new(&file_path);

    // Create parent directories if they don't exist
    if let Some(parent) = json_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json_file = File::create(json_file_path)?;
    serde_json::to_writer_pretty(json_file, config).expect("failed to write config");
    Ok(())
}

fn config_url(key: String, url: String) -> Result<(), Error> {
    let mut config = load_url_config()?;
    let is_update = config.url_map.contains_key(&key);
    config.url_map.insert(key.clone(), url.clone());
    save_url_config(&config)?;

    if is_update {
        println!("updated {} -> {}", key, url);
    } else {
        println!("added {} -> {}", key, url);
    }
    Ok(())
}

fn list_urls() -> Result<(), Error> {
    let url_map = load_url_config()?;

    if url_map.url_map.is_empty() {
        println!("No URLs configured.");
        println!("Add URLs to ~/.config/goto/urls.json");
    } else {
        println!("Available URLs:\n");
        let mut keys: Vec<_> = url_map.url_map.keys().collect();
        keys.sort();
        for key in keys {
            println!("  {} -> {}", key, url_map.url_map[key]);
        }
    }

    Ok(())
}

fn url(url_key: Option<String>) -> Result<(), Error> {
    let key = match url_key {
        Some(k) => k,
        None => return list_urls(),
    };

    let url_map = load_url_config()?;

    if url_map.url_map.contains_key(&key) {
        let url = &url_map.url_map[&key];
        let mut process = Command::new("xdg-open").arg(url).spawn()?;
        process.wait().expect("waiting for command to finish");
        println!("opened {}", url);
    } else {
        println!("url not found for: {}", key);
        println!("\nAvailable keys:");
        let mut keys: Vec<_> = url_map.url_map.keys().collect();
        keys.sort();
        for k in keys {
            println!("  {}", k);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args {
        Cli::Github { commit } => github(commit)?,
        Cli::Travis => travis()?,
        Cli::Rust { search } => rust(search)?,
        Cli::Url { key } => url(key)?,
        Cli::Config { cmd } => match cmd {
            ConfigCmd::Url { key, url } => config_url(key, url)?,
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for parse_git_origin_to_github_url
    mod git_url_parsing {
        use super::*;

        #[test]
        fn parses_ssh_format() {
            let result = parse_git_origin_to_github_url("git@github.com:user/repo.git");
            assert_eq!(result.unwrap(), "https://github.com/user/repo");
        }

        #[test]
        fn parses_ssh_format_without_git_suffix() {
            let result = parse_git_origin_to_github_url("git@github.com:user/repo");
            assert_eq!(result.unwrap(), "https://github.com/user/repo");
        }

        #[test]
        fn parses_https_format() {
            let result = parse_git_origin_to_github_url("https://github.com/user/repo.git");
            assert_eq!(result.unwrap(), "https://github.com/user/repo");
        }

        #[test]
        fn parses_https_format_without_git_suffix() {
            let result = parse_git_origin_to_github_url("https://github.com/user/repo");
            assert_eq!(result.unwrap(), "https://github.com/user/repo");
        }

        #[test]
        fn parses_http_format() {
            let result = parse_git_origin_to_github_url("http://github.com/user/repo.git");
            assert_eq!(result.unwrap(), "http://github.com/user/repo");
        }

        #[test]
        fn rejects_unsupported_format() {
            let result = parse_git_origin_to_github_url("svn://example.com/repo");
            assert!(result.is_err());
        }
    }

    // Tests for UrlsMap serialization
    mod urls_map {
        use super::*;

        #[test]
        fn serializes_to_json() {
            let mut map = HashMap::new();
            map.insert("gh".to_string(), "https://github.com".to_string());
            let urls = UrlsMap { url_map: map };

            let json = serde_json::to_string(&urls).unwrap();
            assert!(json.contains("\"gh\""));
            assert!(json.contains("https://github.com"));
        }

        #[test]
        fn deserializes_from_json() {
            let json = r#"{"url_map": {"test": "https://test.com"}}"#;
            let urls: UrlsMap = serde_json::from_str(json).unwrap();

            assert_eq!(urls.url_map.get("test").unwrap(), "https://test.com");
        }

        #[test]
        fn handles_empty_map() {
            let json = r#"{"url_map": {}}"#;
            let urls: UrlsMap = serde_json::from_str(json).unwrap();

            assert!(urls.url_map.is_empty());
        }

        #[test]
        fn handles_multiple_entries() {
            let json = r#"{"url_map": {"a": "https://a.com", "b": "https://b.com", "c": "https://c.com"}}"#;
            let urls: UrlsMap = serde_json::from_str(json).unwrap();

            assert_eq!(urls.url_map.len(), 3);
            assert_eq!(urls.url_map.get("a").unwrap(), "https://a.com");
            assert_eq!(urls.url_map.get("b").unwrap(), "https://b.com");
            assert_eq!(urls.url_map.get("c").unwrap(), "https://c.com");
        }
    }

    // Tests for Rust doc URL building
    mod rust_docs {
        use super::*;

        #[test]
        fn builds_search_url() {
            let search = "HashMap";
            let expected = "https://doc.rust-lang.org/std/index.html?search=HashMap";
            assert_eq!(format!("{}{}", RUST_DOC_HTTP, search), expected);
        }

        #[test]
        fn builds_search_url_with_spaces() {
            let search = "Vec push";
            let url = format!("{}{}", RUST_DOC_HTTP, search);
            assert!(url.contains("Vec push"));
        }
    }

    // Tests for Travis URL conversion
    mod travis_url {
        #[test]
        fn converts_github_to_travis() {
            let github_url = "https://github.com/user/repo";
            let travis_url = github_url.replace("github.com", "travis-ci.org");
            assert_eq!(travis_url, "https://travis-ci.org/user/repo");
        }
    }
}

extern crate dotenv;
extern crate reqwest;

mod error;

use std::env;
use std::io::Read;

use dotenv::dotenv;

pub use error::Error;

pub fn fetch_version(pkg_name: String, version: String) -> Result<String, Error> {
    let url = build_url(&pkg_name, &version);

    let mut response = reqwest::get(&url)?;
    if !response.status().is_success() {
        println!(
            "there was an error fetching {}@{}: {}",
            &pkg_name,
            &version,
            response.status()
        );
        return Err(Error::Response(response));
    }

    let mut readme = String::new();
    response.read_to_string(&mut readme)?;
    Ok(readme)
}

pub fn fetch_latest(pkg_name: String) -> Result<String, Error> {
    fetch_version(pkg_name, String::from("latest"))
}

fn build_url(pkg_name: &str, version: &str) -> String {
    dotenv().ok();
    let host = env::var("READ_FILE_HOST").expect("you must provide a READ_FILE_HOST url");

    format!(
        "{}/v1/files/file/packages/{}/{}/versions/version/{}/readme.html",
        host,
        first_char(pkg_name),
        pkg_name,
        version,
    ).to_string()
}

fn first_char(pkg_name: &str) -> char {
    return pkg_name.bytes().next().unwrap() as char;
}

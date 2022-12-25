#![allow(dead_code, unused)]
use static_init::dynamic;
mod collector;
use const_format::concatcp;
use collector::*;

include!("structs.rs");

#[dynamic]
static CLIENT: reqwest::Client = reqwest::Client::new();
const MOVIES_PER_PAGE: usize = 32;
const DOMAIN: &str = "https://sflix.to";
const MOVIES_URL: &str = concatcp!(DOMAIN, "/movie?page=");
const TVSHOWS_URL: &str = concatcp!(DOMAIN, "/tv-show?page=");

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("socks5://localhost:9050").unwrap())
    .build().unwrap()
}
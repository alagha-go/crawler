use crate::CLIENT;
use crate::*;
use html_parser::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


include!("request.rs");
include!("collect.rs");
include!("content.rs");
include!("elements.rs");
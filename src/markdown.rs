use std::path::PathBuf;
use axum::http::StatusCode;
use gray_matter::Matter;
use gray_matter::engine::YAML;

use pulldown_cmark::{Parser, Options, html};

use log::{info, error, trace};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Frontmatter {
    Title: String,
    Author: String,
    Date: String,
    Slug: String
}

impl Frontmatter {
    /// Attempt to create an instance of front-matter from a raw post, returning
    /// both the front-matter (if present) and the content, split. If the input 
    /// has no valid Frontmatter, `None` is returned.
    pub fn parse_frontmatter(input: String) -> Option<(Self, String)> {
        let matter_engine = Matter::<YAML>::new();
        match matter_engine.parse_with_struct::<Self>(input.as_str()) {
            Some(post) => {
                trace!("Loaded front-matter from input.");
                let data = post.data;
                let content = post.content;
    
                Some((data, content))
            },
            None => {
                error!("Failed to parse any front-matter from provided input.");
                None
            },
        }
    }

    //Extractors. These return a copy of the current values.
    pub fn get_title(&self) -> String {
        self.Title.clone()
    }

    pub fn get_slug(&self) -> String {
        self.Slug.clone()
    }

    pub fn get_author(&self) -> String {
        self.Author.clone()
    } 

    pub fn get_date(&self) -> String {
        self.Date.clone()
    } 
}


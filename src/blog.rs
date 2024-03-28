use std::collections::HashMap;

use axum::http::StatusCode;
use log::{error, info};

use crate::markdown::Frontmatter;

/// A collection of all Blogposts we know of. The key for the hashmap is the value
/// of the blogpost's slug, which MUST be unique.
#[derive(Clone)]
pub struct Blogposts {
    posts: HashMap<String, Blogpost>
}

/* TODO: All of the blog rendering stuff needs to be refactored.
    The methods we're using for rendering the pages at request time needs to
    somehow be re-done. We should pre-scan the blog/ folder at launch and add 
    all items detected with valid Blog content to the Blogposts struct to track 
    them, by their post slug. Then, re-scanning the blog/ directory will let us 
    update this struct and referenced parsed blogposts.
*/

#[derive(Clone)]
pub struct Blogpost {
    id: String,
    front_matter: Frontmatter,
    content: String
}

impl Blogposts { 
    /// Initialize a new, empty Blog container. You can fill this with Blogposts
    /// later.
    fn new() -> Self {
        Blogposts {
            posts: HashMap::new(),
        }
    }

    /// Attempt to fill a container with blogposts, from the source directory.
    /// Identical to doing the above, and then filling it manually.
    pub fn read_sources() -> Self {
        let posts = Self::new();

        let files = std::fs::read_dir(".\\blog").and_then(|files| {
            for file in files {
                let f = file.unwrap();
                let f_name = f.file_name();
                let f_path = f.path();

                

                info!("Blogpost detected: {}; Name: {:?}", f_path.display(), f_name);
            };

            Ok(())
        });

        posts
    }
    //Get a post from the list by its slug.
    pub fn get_post(&self, id: String) {
        self.posts.get(&id);
    }
}

impl Blogpost {
    pub fn new(raw_post: String) -> Result<Self, StatusCode> {
        let (front_matter, raw_content) = match Frontmatter::parse_frontmatter(raw_post) {
            Some((fm,c)) => {
                (fm,c)
            },
            None => {
                error!("Failed to parse blogpost: No valid front-matter.");
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            },
        };

        Ok(Self {
            id: front_matter.get_slug().to_string(),
            front_matter,
            content: raw_content,
        })
    }
    
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_frontmatter(&self) -> &Frontmatter {
        &self.front_matter
    }

    pub fn get_post(&self) -> &String {
        &self.content
    }
}

/// Enumerate all existing raw blogposts, and convert them into HTML pages.
/// Returns a Blogposts object, which contains a map of all Blogposts.
pub fn init_blog() -> Result<Blogposts, ()> {

    Err(())
}
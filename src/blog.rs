use std::{collections::HashMap, io::Read, path::PathBuf};

use axum::http::StatusCode;
use log::{error, info, trace};

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

#[derive(Clone, Debug)]
pub struct Blogpost {
    id: String,
    front_matter: Option<Frontmatter>,
    content: String
}

impl Blogposts { 
    /// Initialize a new, empty Blog container. You can fill this with Blogposts
    /// later.
    pub fn new() -> Self {
        Blogposts {
            posts: HashMap::new(),
        }
    }

    /// Attempt to fill a container with blogposts, from the source directory.
    pub async fn read_sources(mut self) -> Self {

        let files = std::fs::read_dir("./blog").and_then(|files| {
            for file in files {
                let f = file.unwrap();
                let f_name = f.file_name();
                let f_path = f.path();

                info!("Blogpost detected, name: {:?}", f_name);
                //Read in and parse the blogpost.
                let post = Blogpost::new_from_file(f_path);
                trace!("Built post: {:?}", post);
                //self.posts.insert("", v)
            };

            Ok(())
        });

        self
    }
    //Get a post from the list by its slug.
    pub fn get_post_by_slug(&self, id: String) -> Option<&Blogpost> {
        self.posts.get(&id)
    }
}

impl Blogpost {
    //Create a new blogpost, from an input string.
    pub fn new_from_string(raw_post: String) -> Result<Self, StatusCode> {
        let (fm, c) = Self::parse_frontmatter(raw_post)?;
        
        let post = Blogpost {
            id: fm.get_slug(),
            front_matter: Some(fm),
            content: c,
        };

        Ok(post)
    }

    //Attempt to read in a new blogpost, from the given filepath.
    pub fn new_from_file(path: PathBuf) -> Result<Self, StatusCode> {
        trace!("Trying to read blogpost from {:?}...", path);

        let mut f_post = std::fs::File::open(path)
            .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
        let mut buf:String = String::new();

        f_post.read_to_string(&mut buf).unwrap();
        trace!("Read file content: {}", buf);
        Self::new_from_string(buf)
    }

    pub fn parse_frontmatter(raw_post: String) -> Result<(Frontmatter, String), StatusCode> {
        let (front_matter, content) = match Frontmatter::parse_frontmatter(raw_post) {
            Some((fm,c)) => {
                trace!("Parsed blogpost!");
                (fm,c)
            },
            None => {
                error!("Failed to parse blogpost: No valid front-matter.");
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            },
        };

        Ok((front_matter, content))
    }
    
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_frontmatter(&self) -> Option<Frontmatter> {
        self.front_matter.clone()
    }

    pub fn get_post(&self) -> String {
        self.content.clone()
    }
}

/// Enumerate all existing raw blogposts, and convert them into HTML pages.
/// Returns a Blogposts object, which contains a map of all Blogposts.
pub fn init_blog() -> Result<Blogposts, ()> {

    Err(())
}
use std::{collections::HashMap, path::PathBuf};
use glob::glob;
use pulldown_cmark::{html, Parser};

/// Markdown convenience functions. Used to render user-editable Markdown
/// content into HTML for display.

/// The Markdown context. Represents all Markdown templates that currently
/// exit in the `indir`. Defines the `outdir`, where parsed Markdown will be
/// saved.
#[derive(Clone, Debug)]
pub struct MarkdownCtx {
    pub markdown: HashMap<String, PathBuf>, // A list of all Markdown files available.
    pub autogenerate: bool,              //Should we watch indir and auto-generate outfiles?
}

impl MarkdownCtx {
    fn new() -> Self {
        //Create a map...
        let mut markdown_map = HashMap::new();
        
        //Populate it with Markdown file paths.
        //The Map is populated as <Name, Filepath>
        //Paths are relative the working dir.
       Self::discover(&mut markdown_map, "app/content/**/*.*");

        MarkdownCtx {
            markdown: markdown_map,
            autogenerate: false, //TODO: NYI
        }
    }

    /// Used to perform the directory scan.
    fn discover(map: &mut HashMap<String, PathBuf>, dir: &str) {
        for files in glob(dir).unwrap() {
            match files {
                Ok(f) => {
                    info!("Found items: {:?}", f);
                    let filepath = PathBuf::from(&f);
                    let file = filepath.file_name().unwrap().to_string_lossy().to_string();
                    
                    
                    map.insert(file, filepath);
                },
                Err(e) => todo!(),
            }
            
        }
    }

    /// Scan the Content directory, and repopulate the map of known .md files.
    pub fn rediscover(&self) {

    }
}
/// Turn unparsed Markdown text into parsed HTML, ready to insert in a template.
/// Takes an input String, and an output buffer to write the parsed string to.
/// This is used internally to perform the final render step and actually
/// performs the conversion.
fn parse(input: String, mut buf: String) {
    let parser = Parser::new(&input);
    html::push_html(&mut buf, parser);
}

pub fn init_state() -> MarkdownCtx {
    MarkdownCtx::new()
}
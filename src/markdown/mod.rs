use std::{collections::HashMap, path::PathBuf};

/// Markdown convenience functions. Used to render user-editable Markdown
/// content into HTML for display.
use pulldown_cmark::{html, Parser};

/// The Markdown context. Represents all Markdown templates that currently
/// exit in the `indir`. Defines the `outdir`, where parsed Markdown will be
/// saved.
#[derive(Clone)]
pub struct MarkdownCtx {
    pub templates: HashMap<i64, String>, // A list of all Markdown files available.
    pub autogenerate: bool,              //Should we watch indir and auto-generate outfiles?
    pub indir: PathBuf,                  //The input directory, path relative to the binary.
    pub outdir: PathBuf,                 //The output directlry, also relative to the binary.
}

impl MarkdownCtx {
    fn new() -> Self {
        let template_map = HashMap::new();

        let relative_dir = std::env::current_dir().unwrap();
        
        let indir = PathBuf::from("app/content/");
        let outdir = PathBuf::from("app/live");

        MarkdownCtx {
            templates: template_map,
            autogenerate: false, //TODO: NYI
            indir: relative_dir.clone().join(indir),
            outdir: relative_dir.clone().join(outdir)
        }
    }
}
/// Turn unparsed Markdown into parsed HTML, ready to insert in a template.
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
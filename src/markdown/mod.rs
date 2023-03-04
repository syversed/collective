/// Markdown convenience functions. Used to render user-editable Markdown
/// content into HTML for display.
use pulldown_cmark::{html, Parser};

/// Turn unparsed Markdown into parsed HTML, ready to insert in a template.
/// Takes an input String, and an output buffer to write the parsed string to.
fn parse(input: String, mut buf: String) {
    let parser = Parser::new(&input);
    html::push_html(&mut buf, parser);
}

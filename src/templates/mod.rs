use tera::Tera;
use tera::Error;

/// A Template Context. Should be attached to a Router.
/// Provides usage of the app's Tera templates.
#[derive(Clone)]
pub struct TemplateCtx {
    engine: Tera,
}

impl TemplateCtx {
    /// Create a new TemplateCtx, from a provided Tera engine.
    pub fn new(engine: Tera) -> Self {
        TemplateCtx { 
            engine
        }
    }

    pub fn get_engine(self) -> Tera {
        self.engine
    }
}

/// Perform a load of templates from the specified path.
/// The filename wildcard `/*.html` is applied automatically to the
/// provided `path`.
pub fn load_templates(path: &str) -> Result<TemplateCtx, tera::Error> {
    info!("Loading templates from path: {}", &path);

    let template_path = format!("{}/**/*.html", path);
    let tera = match Tera::new(&template_path) {
        Ok(t) => {
            t
        },
        Err(e) => {
            error!("Tera failed in load_templates call: {}", e);
            return Err(e)
        },
    };

    info!("Tera loaded. File glob: {}", &template_path);
    let template_names = tera.get_template_names();

    if template_names.peekable().peek().is_none() {
        error!("The specified directory has no Templates. Check directory paths.");
        return Err(tera::Error::msg("loaded directory is empty"))
    } else {
        for template in tera.get_template_names() {
            info!("Detected template: {}", template);
        }
    }

    Ok(TemplateCtx::new(tera))
}
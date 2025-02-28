// app/src/template.rs

// dependencies
use crate::configuration::AppConfig;
use pulldown_cmark::{Options, Parser, html::push_html};
use tera::{Error, Tera};

// utility function to convert markdown content to HTML
pub fn markdown_to_html(md_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md_content, options);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    html_output
}

// constructor to compile Tera templates
pub fn compile_templates(config: &AppConfig) -> Result<Tera, Error> {
    let templates_dir = config.templates.dir.as_ref();
    let templates_pattern = format!("{}/**/*", templates_dir);
    let mut templates = Tera::new(&templates_pattern)?;
    templates.add_template_files(vec![(format!("{}/base.html", templates_dir), Some("base"))])?;
    Ok(templates)
}

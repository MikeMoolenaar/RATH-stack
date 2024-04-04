use axum::response::Html;
use html_minifier::HTMLMinifierError;
use minijinja::{context, Environment};
use serde::ser::Serialize;
use std::{error::Error, sync::OnceLock};

pub static SHARED_JINJA_ENV: OnceLock<Environment> = OnceLock::new();

pub fn render_html<S: Serialize>(template_name: &str, context: S, boosted: bool) -> Option<Html<String>> {
    match render(template_name, "body", context, boosted) {
        Ok(html) => Some(html),
        Err(err) => {
            println!("Error rendering html: {}", err);
            return Html(String::from("Woopsie! Something broke!")).into();
        }
    }
}

pub fn render_block<S: Serialize>(template_name: &str, block_name: &str, context: S) -> Option<Html<String>> {
    match render(template_name, block_name, context, true) {
        Ok(html) => Some(html),
        Err(err) => {
            println!("Error rendering block: {}", err);
            return Html(String::from("Woopsie! Something broke!")).into();
        }
    }
}

pub fn render_str<S: Serialize>(template_raw: &str, context: S) -> Option<Html<String>> {
    match SHARED_JINJA_ENV
        .get()
        .expect("Jinja environment not initialized!")
        .render_str(template_raw, context)
    {
        Ok(str) => Some(Html(str)),
        Err(err) => {
            println!("Error rendering string: {}", err);
            return Html(String::from("Woopsie! Something broke!")).into();
        }
    }
}

fn render<S: Serialize>(
    template_name: &str,
    block_name: &str,
    context: S,
    boosted: bool,
) -> Result<Html<String>, Box<dyn Error>> {
    let tpl = SHARED_JINJA_ENV
        .get()
        .expect("Jinja environment not initialized!")
        .get_template(template_name)?;

    if boosted {
        let title = tpl.eval_to_state(context!())?.render_block("title")?;
        let body = tpl.eval_to_state(context)?.render_block(block_name)?;
        let content = format!("<title>{}</title>\n{}", title, body);
        return Ok(minify_html(&content)?);
    } else {
        let content = tpl.render(context)?;
        return Ok(minify_html(&content)?);
    }
}

fn minify_html(html: &str) -> Result<Html<String>, HTMLMinifierError> {
    let html = html_minifier::minify(html)?;
    return Ok(Html(html));
}

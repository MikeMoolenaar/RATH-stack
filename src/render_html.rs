use axum::response::Html;
use minify_html::{minify, Cfg};
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

// TODO: Improve error handling
pub fn render_html_str<S: Serialize>(template_raw: &str, context: S) -> Result<Html<String>, Box<dyn Error>> {
    let template = SHARED_JINJA_ENV
        .get()
        .expect("Jinja environment not initialized!")
        .render_str(template_raw, context)?;
    // Minijiinja does not escape html when using render()
    let str = v_htmlescape::escape(template.as_str());
    return Ok(Html(str.to_string()));
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

fn minify_html(html: &str) -> Result<Html<String>, std::string::FromUtf8Error> {
    let bytes = html.as_bytes();
    let cfg = Cfg::spec_compliant();
    let minified = minify(bytes, &cfg);
    return Ok(Html(String::from_utf8(minified)?));
}

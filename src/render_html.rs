use std::error::Error;

use axum::response::Html;
use minify_html::{minify, Cfg};
use minijinja::{context, Environment};
use serde::ser::Serialize;

pub fn render_html<S: Serialize>(
    template_name: &str,
    context: S,
    jinja_env: &Environment,
    boosted: bool,
) -> Option<Html<String>> {
    match render(template_name, "body", context, jinja_env, boosted) {
        Ok(html) => Some(html),
        Err(err) => {
            println!("Error rendering html: {}", err);
            return Html(String::from("Woopsie! Something broke!")).into();
        }
    }
}

pub fn render_block<S: Serialize>(
    template_name: &str,
    block_name: &str,
    context: S,
    jinja_env: &Environment,
) -> Option<Html<String>> {
    match render(template_name, block_name, context, jinja_env, false) {
        Ok(html) => Some(html),
        Err(err) => {
            println!("Error rendering block: {}", err);
            return Html(String::from("Woopsie! Something broke!")).into();
        }
    }
}

fn render<S: Serialize>(
    template_name: &str,
    block_name: &str,
    context: S,
    jinja_env: &Environment,
    boosted: bool,
) -> Result<Html<String>, Box<dyn Error>> {
    // TODO Use global jinja_env so we don't have to always pass it
    //   https://github.com/photino/zino/blob/main/zino-core/src/view/minijinja.rs
    let tpl = jinja_env.get_template(template_name)?;

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

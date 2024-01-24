use axum::response::Html;
use minify_html::{minify, Cfg};
use minijinja::{context, Environment};
use serde::ser::Serialize;

fn minify_html(html: &str) -> Html<String> {
    let bytes = html.as_bytes();
    let cfg = Cfg::spec_compliant();
    let minified = minify(bytes, &cfg);
    return Html(String::from_utf8(minified).unwrap());
}

pub fn render_html<S: Serialize>(
    template_name: &str,
    context: S,
    jinja_env: &Environment,
    boosted: bool,
) -> Option<Html<String>> {
    // TODO Replace unwraps with better error handling
    // TODO Use global jinja_env so we don't have to always pass it
    //   https://github.com/photino/zino/blob/main/zino-core/src/view/minijinja.rs
    let tpl = jinja_env.get_template(template_name).unwrap();

    if boosted {
        let title = tpl.eval_to_state(context!()).unwrap().render_block("title").unwrap();
        let content = tpl.eval_to_state(context).unwrap().render_block("body").unwrap();
        let combined = format!("<title>{}</title>\n{}", title, content);
        return Some(minify_html(&combined));
    } else {
        let content = tpl.render(context).unwrap();
        return Some(minify_html(&content));
    }
}

pub fn render_block<S: Serialize>(
    template_name: &str,
    block_name: &str,
    context: S,
    jinja_env: &Environment,
) -> Option<Html<String>> {
    let tpl = jinja_env.get_template(template_name).unwrap();

    let title = tpl.eval_to_state(context!()).unwrap().render_block("title").unwrap();
    let content = tpl.eval_to_state(context).unwrap().render_block(block_name).unwrap();
    let combined = format!("<title>{}</title>\n{}", title, content);
    return Some(minify_html(&combined));
}

use minify_html::{Cfg, minify};
use std::collections::HashMap;
use tera::{Context, Tera};

pub fn render_template(title: &str, content: &str, is_production: bool) -> String {
    let tera = Tera::new("templates/*.html").expect("Errore nel caricamento dei template");

    let mut context = Context::new();
    context.insert("title", title);
    context.insert("content", content);

    let html = tera
        .render("base.html", &context)
        .expect("Errore nel rendering");

    // Se siamo in produzione, minifichiamo l'HTML
    if is_production {
        let minified = minify(html.as_bytes(), &Cfg::new());
        return String::from_utf8(minified).expect("Errore nella minificazione HTML");
    }

    html
}

pub fn render_blog(posts: &Vec<HashMap<String, String>>) -> String {
    let tera = Tera::new("templates/*.html").expect("Errore nel caricamento dei template");

    let mut context = Context::new();
    context.insert("posts", posts);

    tera.render("blog.html", &context)
        .expect("Errore nel rendering della pagina Blog")
}

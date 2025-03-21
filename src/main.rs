mod file_io;
mod markdown;
mod server;
mod template;

use std::env;

use serde::Deserialize;
use tokio::runtime::Runtime;

/// Struttura per leggere il frontmatter YAML
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
}

/// Estrae il titolo dal frontmatter YAML nei file Markdown
fn extract_title(content: &str) -> (Option<String>, &str) {
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let yaml = &content[3..3 + end];
            let frontmatter: Result<Frontmatter, _> = serde_yaml::from_str(yaml);
            let body = &content[3 + end + 3..]; // Il resto del file è il vero contenuto Markdown

            if let Ok(frontmatter) = frontmatter {
                return (Some(frontmatter.title), body);
            }
        }
    }
    (None, content) // Se il YAML non è presente, ritorniamo None e il contenuto completo
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let is_production = args.contains(&"--prod".to_string());

    println!("📜 Generatore di Siti Statici in Rust!");

    generate_site();

    if is_production {
        println!("🚀 Sito generato in modalità produzione! (Nessun server avviato)");
    } else {
        // Avviamo l'hot reload
        let (reload_tx, _reload_rx) = std::sync::mpsc::channel();
        server::start_hot_reload(reload_tx);
        let rt = Runtime::new().unwrap();
        rt.block_on(server::start_server());
    }
}

/// Funzione per generare il sito statico
fn generate_site() {
    use std::collections::HashMap;
    let is_production = env::args().any(|arg| arg == "--prod");

    std::fs::create_dir_all("dist").expect("Errore nella creazione della cartella dist/");

    // Generazione delle pagine statiche
    let page_files = file_io::get_markdown_files("content/pages");
    for file in page_files {
        let md_content = file_io::read_file(file.to_str().unwrap());
        let (title, clean_md) = extract_title(&md_content);
        let html_content = markdown::markdown_to_html(clean_md);

        let page_title = title.unwrap_or_else(|| "Senza titolo".to_string()); // Usa il titolo del YAML, altrimenti un fallback
        let final_page = template::render_template(&page_title, &html_content, is_production);

        let filename = file.file_stem().unwrap().to_str().unwrap().to_string();
        let output_path = format!("{}.html", filename);
        file_io::save_html(&output_path, &final_page);
    }

    // Generazione dei post del blog
    let blog_files = file_io::get_markdown_files("content/blog");
    let mut posts = Vec::new();

    for file in &blog_files {
        let md_content = file_io::read_file(file.to_str().unwrap());
        let (title, clean_md) = extract_title(&md_content);
        let html_content = markdown::markdown_to_html(clean_md);

        let post_title = title.unwrap_or_else(|| "Post senza titolo".to_string());
        let final_page = template::render_template(&post_title, &html_content, is_production);

        let post_filename = file.file_stem().unwrap().to_str().unwrap().to_string();
        let output_path = format!("blog/{}.html", post_filename);
        file_io::save_html(&output_path, &final_page);

        posts.push(HashMap::from([
            ("title".to_string(), post_title),
            ("url".to_string(), format!("blog/{}.html", post_filename)),
        ]));
    }

    // Genera la pagina Blog
    let blog_page = template::render_blog(&posts);
    file_io::save_html("blog.html", &blog_page);

    // Genera la Homepage
    let homepage_md = file_io::read_file("content/pages/home.md");
    let (homepage_title, homepage_clean_md) = extract_title(&homepage_md);
    let homepage_title = homepage_title.unwrap_or_else(|| "Home".to_string()); // Usa "Home" come titolo di default
    let homepage_html = markdown::markdown_to_html(homepage_clean_md);
    let homepage_final = template::render_template(&homepage_title, &homepage_html, is_production);
    file_io::save_html("index.html", &homepage_final);

    // Copia i file statici (CSS, immagini, ecc.)
    file_io::copy_static_files();

    file_io::copy_images();

    println!("✅ Sito generato con successo in 'dist/'!");
}

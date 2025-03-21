use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Legge tutti i file Markdown in una cartella e restituisce i loro percorsi
pub fn get_markdown_files(folder: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if Path::new(folder).exists() {
        for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();
            if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }

    files
}

pub fn save_html(filename: &str, content: &str) {
    let clean_filename = filename.trim_start_matches("/"); // Evita doppie "/"
    let output_path = Path::new("dist").join(clean_filename);

    // Assicura che la cartella di destinazione esista
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Errore nella creazione delle cartelle necessarie");
    }

    fs::write(&output_path, content).expect("Errore nella scrittura del file HTML");
}

pub fn copy_static_files() {
    let static_dir = Path::new("static/");
    let dest_dir = Path::new("dist/");

    if static_dir.exists() {
        for entry in fs::read_dir(static_dir).expect("Errore nella lettura della cartella static/")
        {
            let entry = entry.expect("Errore nella lettura del file statico");
            let src_path = entry.path();
            let dest_path = dest_dir.join(entry.file_name());

            fs::copy(&src_path, &dest_path).expect("Errore nella copia dei file statici");
        }
    }
}
pub fn read_file(path: &str) -> String {
    fs::read_to_string(Path::new(path)).expect(&format!("Errore nella lettura del file {}", path))
}
/// Copia le immagini dalla cartella `content/images/` a `dist/images/`
pub fn copy_images() {
    let src_dir = Path::new("content/images/");
    let dest_dir = Path::new("dist/images/");

    if src_dir.exists() {
        fs::create_dir_all(dest_dir).expect("Errore nella creazione della cartella images/");
        for entry in fs::read_dir(src_dir).expect("Errore nella lettura della cartella images/") {
            let entry = entry.expect("Errore nella lettura del file immagine");
            let src_path = entry.path();
            let dest_path = dest_dir.join(entry.file_name());

            fs::copy(&src_path, &dest_path).expect("Errore nella copia dell'immagine");
        }
    }
}

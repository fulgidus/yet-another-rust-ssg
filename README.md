# 🦀 Yet Another Rust SSG

A minimal static site generator built in Rust — not to compete with existing ones, but to learn the language by building something useful (and fun).

> ⚠️ This project is not production-grade. It’s a personal learning experiment.
> The code is simple, the structure is primitive, and that’s the point.

---

## ✨ Features

- 📝 Parses Markdown (`.md`) files with optional YAML frontmatter
- 🧠 Uses [Tera](https://tera.netlify.app/) templates for rendering pages
- 🌙 Supports a modern dark mode with toggle switch
- 🔁 Hot reload with file watcher and WebSocket live refresh
- 🗂️ Blog + page routing system (`/blog/`, `/about.html`, etc.)
- ⚙️ `--prod` mode for minified HTML and deploy-ready output
- 🌍 Simple dev server via [warp](https://github.com/seanmonstar/warp)

---

## 📂 Project Structure

```
yet_another_rust_ssg/
 ├── content/ # Markdown source files
 │ ├── pages/ # Static pages (about, contact, etc.)
 │ ├── blog/ # Blog posts
 ├── templates/ # HTML templates (Tera)
 │ ├── base.html
 │ ├── page.html
 │ ├── blog.html
 ├── static/ # Static assets (CSS, images)
 │ ├── styles.css
 ├── dist/ # Generated HTML output
 ├── src/ # Rust codebase
 ├── Cargo.toml # Rust dependencies
```

---

## 🚀 Getting Started

### 🔧 Requirements

- Rust (latest stable)
- Cargo

### 🛠️ Build and Run in Development

```bash
cargo run
```
This will:

-    Generate the site in the dist/ folder
-    Start a local server at http://localhost:3000
-    Watch for content/template changes and reload the browser

### 🧪 Run in Production

```bash
cargo run -- --prod
```

### 🧠 Why Build This?

Because learning Rust by building something from scratch is incredibly rewarding. This project is not meant to replace Hugo, Zola, or Astro — it’s a humble, self-contained SSG that taught me a lot about Rust, web tooling, and project structuring.

### 🐾 License

This project is released under the MIT License.
Feel free to fork, hack, or remix it for your own learning journey. 🎓
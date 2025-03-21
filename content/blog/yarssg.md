---
title: "Yet Another Rust SSG"
tags: ["Rust", "Static Site Generator", "SSG", "Personal Project"]
---

# Yet Another Rust SSG  

## 🚀 Why Another SSG?  

The world doesn’t need another Static Site Generator. **Hugo** is lightning fast, **Jekyll** has been around forever, and modern tools like **Astro** and **Eleventy** are packed with features. So why build one in Rust?  

Simple: **because I wanted to write something in Rust.**  

This isn’t an attempt to compete with existing SSGs. It’s not here to revolutionize static site generation. Instead, it’s a personal project—an exercise in learning Rust while building something useful. And in the process, I gained a deeper understanding of file handling, templating, and server-side rendering, all within Rust’s safe and efficient ecosystem.  

---

## 🦀 Learning Rust, One Problem at a Time  

Building an SSG from scratch was a great way to explore Rust’s capabilities. I had to deal with:  

- **Reading and parsing Markdown files**  
- **Transforming content into HTML using templates**  
- **Managing file I/O efficiently**  
- **Serving the generated site locally with a simple web server**  
- **Implementing a dark mode toggle for fun**  

Each of these challenges forced me to dive into Rust’s libraries and ecosystem. I learned how to handle errors gracefully, how Rust’s ownership model works in practice, and how to structure a small but useful project.  

---

## 🏗️ Keeping It Simple  

Unlike production-ready SSGs, **Yet Another Rust SSG** is deliberately minimal. It takes a folder of Markdown files, converts them to HTML, applies a template, and outputs a `dist/` directory ready to be deployed. There’s no complex plugin system, no deep configurability—just a straightforward pipeline that does what it needs to do.  

Along the way, I added:  
✅ A simple templating system for inserting content into HTML  
✅ A development server to preview changes locally  
✅ A `--prod` flag to generate minified HTML for production  
✅ A dark mode toggle, because why not?  

Everything is built with Rust’s safety and speed in mind, but I didn’t obsess over making it the fastest or most efficient SSG out there. The goal was always **learning first, performance second**.  

---

## 🌍 Deploying the Site  

One of the nice things about static sites is how easy they are to deploy. After generating the HTML, I can push it to **GitHub Pages**, upload it to **Netlify**, or even serve it from an **S3 bucket**. No backend, no database, just pure HTML and CSS.  

This simplicity is part of what makes static sites so appealing. Even with a hand-rolled SSG, I get all the benefits of speed, security, and portability.  

---

## 🎯 What’s Next?  

Will I keep improving this project? Maybe. There are plenty of things I could add:  

- **An RSS feed for blog posts**  
- **Tag and category support**  
- **Multi-threaded processing for faster builds**  
- **A plugin system for custom extensions**  

But at the end of the day, **this project has already served its purpose**. It gave me an excuse to write Rust, to solve real-world problems, and to build something tangible. And that’s exactly what I wanted from it.  

So, if you’re thinking about writing your own SSG—not because the world needs it, but because you want to learn—**go for it**. You might be surprised by how much you learn along the way.  

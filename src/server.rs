use futures_util::sink::SinkExt;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::{thread, time::Duration};
use tokio::sync::broadcast;
use warp::Filter;
use warp::ws::Message;

/// Serve i file statici e WebSocket per il live reload
pub async fn start_server() {
    let dist_path = "dist";
    let (reload_tx, _reload_rx) = tokio::sync::broadcast::channel(10);

    let static_files = warp::fs::dir(dist_path);

    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let reload_tx = reload_tx.clone();
            ws.on_upgrade(move |socket| async move {
                websocket_handler(socket, reload_tx).await;
            })
        });

    let routes = static_files
        .or(websocket_route
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::OK)));

    println!("🌍 Server avviato su http://localhost:3000 🚀");

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

/// Avvia l'hot reload: monitora `content/` e rigenera il sito quando cambia qualcosa
pub fn start_hot_reload(_reload_tx: Sender<()>) {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())
        .expect("Errore nella creazione del watcher");
    watcher
        .watch(Path::new("content/"), RecursiveMode::Recursive)
        .expect("Errore nel monitoraggio della cartella");

    thread::spawn(move || {
        loop {
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(event) => {
                    if let Ok(event) = event {
                        if let Event {
                            kind: EventKind::Modify(ModifyKind::Data(_)),
                            paths,
                            ..
                        } = event
                        {
                            if let Some(path) = paths.get(0) {
                                println!("🔄 Modifica rilevata in {:?}, rigenero il sito...", path);

                                let _ = std::process::Command::new("cargo")
                                    .arg("run")
                                    .status()
                                    .expect("Errore durante la rigenerazione del sito");
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });

    println!("👀 Hot reload attivo: monitora i file Markdown in `content/`!");
}
/// Avvia un server WebSocket per notificare il browser quando i file cambiano
pub async fn websocket_handler(mut socket: warp::ws::WebSocket, tx: broadcast::Sender<()>) {
    let mut rx = tx.subscribe();
    while let Ok(_) = rx.recv().await {
        let _ = socket.send(Message::text("reload")).await;
    }
}

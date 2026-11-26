//! state-o-rama: guess the state from its shape, with friends, against the clock.
//! One binary: HTTP for the page, a WebSocket per player, one task per lobby.

mod lobby;
mod quiz;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use futures_util::{SinkExt, StreamExt};
use lobby::{Cmd, Registry};
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::oneshot;

static INDEX: &str = include_str!("../web/index.html");
static APP: &str = include_str!("../web/app.js");
static STYLE: &str = include_str!("../web/style.css");
static MAPS: &str = include_str!("../web/maps.json");
static ICON: &str = include_str!("../web/icon.svg");

#[tokio::main]
async fn main() {
    let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:8050".into());
    let reg: Registry = Default::default();
    let app = Router::new()
        .route("/", get(|| async { page() }))
        .route("/l/{code}", get(lobby_page))
        .route("/new", get(new_lobby))
        .route("/app.js", get(|| async { asset("text/javascript", APP) }))
        .route("/style.css", get(|| async { asset("text/css", STYLE) }))
        .route("/maps.json", get(|| async { asset("application/json", MAPS) }))
        .route("/icon.svg", get(|| async { asset("image/svg+xml", ICON) }))
        .route("/qr/{code}", get(qr))
        .route("/healthz", get(|| async { "ok" }))
        .route("/ws/{code}", get(ws))
        .with_state(reg);
    let listener = tokio::net::TcpListener::bind(&bind).await.expect("bind");
    println!("state-o-rama on http://{bind}");
    axum::serve(listener, app)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.ok(); })
        .await
        .unwrap();
}

fn page() -> Response {
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8"), (header::CACHE_CONTROL, "no-cache")], INDEX).into_response()
}

fn asset(ct: &'static str, body: &'static str) -> Response {
    ([(header::CONTENT_TYPE, ct), (header::CACHE_CONTROL, "no-cache")], body).into_response()
}

fn valid_code(code: &str) -> Option<String> {
    let c = code.to_ascii_uppercase();
    (c.len() >= 3 && c.len() <= 8 && c.chars().all(|ch| ch.is_ascii_alphanumeric())).then_some(c)
}

async fn lobby_page(Path(code): Path<String>) -> Response {
    match valid_code(&code) {
        Some(c) if c == code => page(),
        Some(c) => Redirect::to(&format!("/l/{c}")).into_response(),
        None => Redirect::to("/").into_response(),
    }
}

/// The lobby's link as a QR code, for a room full of phones.
async fn qr(Path(code): Path<String>, headers: axum::http::HeaderMap) -> Response {
    let Some(code) = valid_code(&code) else { return StatusCode::NOT_FOUND.into_response() };
    let host = headers.get(header::HOST).and_then(|h| h.to_str().ok()).unwrap_or("states.brensch.com");
    let scheme = if host.starts_with("127.") || host.starts_with("localhost") { "http" } else { "https" };
    let Ok(q) = qrcode::QrCode::new(format!("{scheme}://{host}/l/{code}")) else { return StatusCode::BAD_REQUEST.into_response() };
    let svg = q.render::<qrcode::render::svg::Color>().quiet_zone(true).min_dimensions(240, 240)
        .dark_color(qrcode::render::svg::Color("#1d2340")).light_color(qrcode::render::svg::Color("#ffffff")).build();
    ([(header::CONTENT_TYPE, "image/svg+xml"), (header::CACHE_CONTROL, "public, max-age=86400")], svg).into_response()
}

/// A fresh lobby code: pronounceable, and not one that's in use.
async fn new_lobby(State(reg): State<Registry>) -> Redirect {
    const C: &[u8] = b"BDFGKLMNPRSTVWZ";
    const V: &[u8] = b"AEIOU";
    let mut rng = rand::rng();
    let code = loop {
        let code: String = (0..5).map(|i| *if i % 2 == 0 { C } else { V }.choose(&mut rng).unwrap() as char).collect();
        if !reg.lock().unwrap().contains_key(&code) {
            break code;
        }
    };
    Redirect::to(&format!("/l/{code}"))
}

#[derive(Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
enum ClientMsg {
    Hello { token: String, name: String },
    Start { rounds: usize },
    Answer { round: usize, choice: String },
    Again,
    Rename { name: String },
}

async fn ws(Path(code): Path<String>, State(reg): State<Registry>, up: WebSocketUpgrade) -> Response {
    let Some(code) = valid_code(&code) else { return StatusCode::NOT_FOUND.into_response() };
    up.max_message_size(4096).on_upgrade(move |sock| session(sock, code, reg))
}

async fn session(sock: WebSocket, code: String, reg: Registry) {
    let (mut tx, mut rx) = sock.split();

    // the first message says who this is
    let hello = tokio::time::timeout(Duration::from_secs(10), rx.next()).await;
    let Ok(Some(Ok(Message::Text(t)))) = hello else { return };
    let Ok(ClientMsg::Hello { token, name }) = serde_json::from_str(&t) else { return };
    if token.len() < 8 || token.len() > 64 {
        return;
    }

    // join, starting the lobby if needed (twice: the lobby may be shutting down as we arrive)
    let mut joined = None;
    for _ in 0..2 {
        let lobby = lobby::get_or_start(&reg, &code);
        let (reply, wait) = oneshot::channel();
        if lobby.send(Cmd::Join { token: token.clone(), name: name.clone(), reply }).await.is_err() {
            continue;
        }
        match wait.await {
            Ok(Ok(j)) => {
                joined = Some((lobby, j));
                break;
            }
            Ok(Err(msg)) => {
                let _ = tx.send(Message::Text(serde_json::json!({"t": "error", "msg": msg}).to_string().into())).await;
                return;
            }
            Err(_) => continue,
        }
    }
    let Some((lobby, mut j)) = joined else { return };
    let welcome = serde_json::json!({"t": "welcome", "you": j.you}).to_string();

    let run = async {
        tx.send(Message::Text(welcome.into())).await.ok()?;
        j.snapshots.mark_changed();
        let mut ping = tokio::time::interval(Duration::from_secs(20));
        loop {
            tokio::select! {
                changed = j.snapshots.changed() => {
                    changed.ok()?;
                    let s = j.snapshots.borrow_and_update().clone();
                    tx.send(Message::Text(s.to_string().into())).await.ok()?;
                }
                msg = rx.next() => {
                    let Message::Text(t) = msg?.ok()? else { continue };
                    let Ok(m) = serde_json::from_str::<ClientMsg>(&t) else { continue };
                    let token = token.clone();
                    let cmd = match m {
                        ClientMsg::Hello { .. } => continue,
                        ClientMsg::Start { rounds } => Cmd::Start { token, rounds },
                        ClientMsg::Answer { round, choice } => Cmd::Answer { token, round: round.wrapping_sub(1), choice },
                        ClientMsg::Again => Cmd::Again { token },
                        ClientMsg::Rename { name } => Cmd::Rename { token, name },
                    };
                    lobby.send(cmd).await.ok()?;
                }
                _ = ping.tick() => { tx.send(Message::Ping(Default::default())).await.ok()?; }
            }
        }
        #[allow(unreachable_code)]
        Some(())
    };
    let _: Option<()> = run.await;
    let _ = lobby.send(Cmd::Leave { token }).await;
}

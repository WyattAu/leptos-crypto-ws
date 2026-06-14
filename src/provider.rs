//! WebSocket provider component and context.

use std::collections::HashMap;
use futures::StreamExt;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, CloseEvent, ErrorEvent};

use crate::types::*;

/// Leptos context key for the crypto WebSocket data.
pub struct CryptoWsKey;

/// Provider component that establishes a Binance WebSocket connection
/// and provides crypto ticker data via Leptos context.
///
/// # Props
///
/// - `symbols`: Vec<String> — Binance symbols to subscribe to (optional, uses DEFAULT_SYMBOLS)
/// - `children`: Children — child components that consume the data
#[component]
pub fn CryptoWsProvider(
    #[prop(optional)] symbols: Option<Vec<String>>,
    children: Children,
) -> impl IntoView {
    let symbols = symbols.unwrap_or_else(|| {
        DEFAULT_SYMBOLS.iter().map(|s| s.to_string()).collect()
    });

    let data = RwSignal::new(HashMap::<String, CryptoTickerEntry>::new());

    // Connect WebSocket on mount
    #[cfg(feature = "hydrate")]
    {
        let data = data.clone();
        let symbols = symbols.clone();

        leptos::task::spawn_local(async move {
            connect_ws(&symbols, data).await;
        });
    }

    // Provide context
    provide_context(CryptoWsData { data: data.into() });

    view! { {children()} }
}

/// Access the crypto WebSocket data from context.
pub fn use_crypto_ws() -> Signal<HashMap<String, CryptoTickerEntry>> {
    use_context::<CryptoWsData>()
        .expect("CryptoWsProvider not found — wrap your app in <CryptoWsProvider>")
        .data
}

/// Internal context holder.
#[derive(Clone)]
struct CryptoWsData {
    data: Signal<HashMap<String, CryptoTickerEntry>>,
}

/// Connect to Binance WebSocket and update the signal.
async fn connect_ws(symbols: &[String], data: RwSignal<HashMap<String, CryptoTickerEntry>>) {
    let url = build_stream_url_for(symbols);

    loop {
        match try_connect(&url, &data).await {
            WSResult::Disconnected => {
                // Reconnect after 3 seconds
                gloo_timers::future::TimeoutFuture::new(3000).await;
            }
            WSResult::Error => {
                // Reconnect after 5 seconds on error
                gloo_timers::future::TimeoutFuture::new(5000).await;
            }
        }
    }
}

enum WSResult {
    Disconnected,
    Error,
}

async fn try_connect(
    url: &str,
    data: &RwSignal<HashMap<String, CryptoTickerEntry>>,
) -> WSResult {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return WSResult::Error,
    };

    let ws = match web_sys::WebSocket::new(url) {
        Ok(ws) => ws,
        Err(_) => return WSResult::Error,
    };

    let ws_clone = ws.clone();
    let data_clone = data.clone();

    // Set up message handler
    let onmessage = Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
        if let Ok(text) = event.data().dyn_into::<js_sys::JsString>() {
            if let Some(text_str) = text.as_string() {
                if let Ok(msg) = serde_json::from_str::<BinanceStreamMessage>(&text_str) {
                    let entry = msg.data.to_entry();
                    data_clone.update(|d| {
                        d.insert(entry.symbol.clone(), entry);
                    });
                }
            }
        }
    });
    ws_clone.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // Set up close handler
    let (tx, rx) = futures::channel::mpsc::unbounded::<()>();
    let tx_clone = tx.clone();
    let onclose = Closure::<dyn Fn(CloseEvent)>::new(move |_event: CloseEvent| {
        let _ = tx_clone.unbounded_send(());
    });
    ws_clone.set_onclose(Some(onclose.as_ref().unchecked_ref()));
    onclose.forget();

    // Set up error handler
    let tx_err = tx.clone();
    let onerror = Closure::<dyn Fn(ErrorEvent)>::new(move |_event: ErrorEvent| {
        let _ = tx_err.unbounded_send(());
    });
    ws_clone.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    // Wait until close
    let _ = rx.into_future().await;

    WSResult::Disconnected
}

fn build_stream_url_for(symbols: &[String]) -> String {
    let streams: Vec<String> = symbols
        .iter()
        .map(|s| format!("{}@ticker", s.to_lowercase()))
        .collect();
    format!(
        "wss://stream.binance.com:9443/stream?streams={}",
        streams.join("/")
    )
}

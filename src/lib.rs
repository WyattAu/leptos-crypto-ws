//! # leptos-crypto-ws
//!
//! Real-time Binance WebSocket client for Leptos.
//!
//! Provides live crypto prices as reactive signals via Binance's combined stream.
//! Handles connection, reconnection, and JSON parsing automatically.
//!
//! ## Features
//!
//! - Connects to Binance combined stream (`btcusdt@ticker`, `ethusdt@ticker`, etc.)
//! - Automatic reconnection on disconnect
//! - Typed structs for Binance 24h ticker payloads
//! - Reactive signal updates via Leptos context
//! - Configurable symbols and update interval
//!
//! ## Usage
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_crypto_ws::{CryptoWsProvider, use_crypto_ws};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <CryptoWsProvider>
//!             <TickerBar/>
//!         </CryptoWsProvider>
//!     }
//! }
//!
//! #[component]
//! fn TickerBar() -> impl IntoView {
//!     let crypto = use_crypto_ws();
//!     view! {
//!         <div class="ticker">
//!             {move || {
//!                 let data = crypto.get();
//!                 data.get("BTCUSDT")
//!                     .and_then(|t| t.price)
//!                     .map(|p| format!("${p:.2}"))
//!                     .unwrap_or_else(|| "--".to_string())
//!             }}
//!         </div>
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod provider;
mod types;

pub use provider::{use_crypto_ws, CryptoWsProvider};
pub use types::CryptoTickerEntry;
